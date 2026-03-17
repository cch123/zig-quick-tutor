package main

import (
	"fmt"
	"runtime"
	"strings"
)

// Go 的内存管理完全由垃圾回收器处理
// 程序员不需要手动分配/释放内存

// 1. 使用 new —— 分配单个值，返回指针
func createPoint() *[2]float64 {
	p := new([2]float64)
	p[0] = 3.0
	p[1] = 4.0
	return p // 安全：GC 会管理生命周期
}

// 2. 使用 make —— 分配 slice、map、channel
func createData() []int {
	data := make([]int, 0, 100) // len=0, cap=100
	for i := 0; i < 10; i++ {
		data = append(data, i+1)
	}
	return data
}

// 3. 逃逸分析演示
//
//go:noinline
func stackAlloc() int {
	// x 不逃逸，分配在栈上
	x := 42
	return x
}

//go:noinline
func heapAlloc() *int {
	// x 逃逸到堆上，因为返回了指针
	x := 42
	return &x
}

// 4. 结构体内存管理 —— 不需要析构函数
type Buffer struct {
	data []byte
	name string
}

func NewBuffer(name string, size int) *Buffer {
	return &Buffer{
		data: make([]byte, size),
		name: name,
	}
}

func (b *Buffer) Fill(ch byte) {
	for i := range b.data {
		b.data[i] = ch
	}
}

func (b *Buffer) String() string {
	return fmt.Sprintf("Buffer(%s, len=%d)", b.name, len(b.data))
}

// 5. copy —— Go 的 memcpy 等价物
func copyDemo() {
	src := []byte("Hello")
	dst := make([]byte, len(src))
	n := copy(dst, src)
	fmt.Printf("copied %d bytes: %s\n", n, dst)

	// 用循环填充（Go 没有内建 memset）
	buf := make([]byte, 10)
	for i := range buf {
		buf[i] = 'A'
	}
	fmt.Printf("filled: %s\n", buf)
}

func main() {
	// 1. new —— 分配单个值
	fmt.Println("=== new ===")
	point := createPoint()
	fmt.Printf("point: (%.1f, %.1f)\n", point[0], point[1])

	// 2. make —— 分配 slice
	fmt.Println("\n=== make ===")
	data := createData()
	fmt.Println("data:", data)

	// make 用于 map
	m := make(map[string]int)
	m["one"] = 1
	m["two"] = 2
	fmt.Println("map:", m)

	// make 用于 channel
	ch := make(chan int, 5)
	ch <- 42
	fmt.Println("channel value:", <-ch)

	// 3. 逃逸分析
	fmt.Println("\n=== escape analysis ===")
	v1 := stackAlloc()
	v2 := heapAlloc()
	fmt.Printf("stack: %d, heap: %d\n", v1, *v2)
	// 使用 go build -gcflags="-m" 可以看到逃逸分析结果

	// 4. Buffer
	fmt.Println("\n=== buffer ===")
	buf := NewBuffer("test", 10)
	buf.Fill('X')
	fmt.Println(buf)
	fmt.Printf("content: %s\n", buf.data)
	// 不需要手动释放！GC 会自动回收

	// 5. copy
	fmt.Println("\n=== copy (memcpy equivalent) ===")
	copyDemo()

	// 6. strings.Builder —— 高效字符串拼接（减少分配）
	fmt.Println("\n=== strings.Builder ===")
	var sb strings.Builder
	for i := 0; i < 5; i++ {
		fmt.Fprintf(&sb, "item_%d ", i)
	}
	fmt.Println(sb.String())

	// 7. GC 统计
	fmt.Println("\n=== GC stats ===")
	var stats runtime.MemStats
	runtime.ReadMemStats(&stats)
	fmt.Printf("Alloc: %d KB\n", stats.Alloc/1024)
	fmt.Printf("TotalAlloc: %d KB\n", stats.TotalAlloc/1024)
	fmt.Printf("NumGC: %d\n", stats.NumGC)

	// Go 的内存管理哲学：
	// - 程序员不需要关心内存释放
	// - GC 自动管理，代价是不确定的延迟和额外内存开销
	// - 逃逸分析优化：编译器尽量在栈上分配
	// - 没有析构函数，需要显式清理的资源使用 defer + Close()
}
