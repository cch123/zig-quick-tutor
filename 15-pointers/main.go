package main

import (
	"fmt"
	"unsafe"
)

func increment(ptr *int) {
	*ptr += 1 // 通过指针修改值
}

func main() {
	// 1. 基本指针
	x := 42
	ptr := &x
	fmt.Printf("x = %d, *ptr = %d\n", x, *ptr)

	// 通过指针修改值
	*ptr = 100
	fmt.Printf("After *ptr = 100: x = %d\n", x)

	// 2. 通过指针传参
	increment(&x)
	fmt.Printf("After increment: x = %d\n", x) // 101

	// 3. new() 在堆上分配
	p := new(int)
	*p = 42
	fmt.Printf("new(int): %d\n", *p)

	// 4. nil 指针
	var nilPtr *int
	fmt.Println("nilPtr is nil:", nilPtr == nil)
	// fmt.Println(*nilPtr) // panic: nil pointer dereference

	// 5. 自动取地址 - 方法调用时
	type Point struct {
		X, Y int
	}
	pt := Point{1, 2}
	// Go 自动取地址：pt.Move() 等价于 (&pt).Move()
	// （如果 Move 是指针接收者方法）
	fmt.Printf("Point: %+v\n", pt)

	// 6. 指针和切片
	arr := [5]int{10, 20, 30, 40, 50}
	slice := arr[:]
	// 修改切片会影响底层数组
	slice[0] = 99
	fmt.Println("arr[0] after slice mod:", arr[0]) // 99

	// 7. Go 没有指针算术（除非用 unsafe）
	data := [4]byte{0x01, 0x00, 0x00, 0x00}
	// 使用 unsafe 进行类型转换（不推荐）
	intVal := *(*uint32)(unsafe.Pointer(&data[0]))
	fmt.Printf("Bytes as uint32: %d\n", intVal)

	// unsafe.Pointer 进行指针算术
	nums := [3]int{100, 200, 300}
	base := unsafe.Pointer(&nums[0])
	second := (*int)(unsafe.Add(base, unsafe.Sizeof(nums[0])))
	fmt.Printf("nums[1] via pointer arithmetic: %d\n", *second)

	// 8. 函数返回局部变量的指针是安全的（逃逸分析）
	makePtr := func() *int {
		v := 42
		return &v // Go 会自动将 v 分配到堆上
	}
	fmt.Println("Escaped ptr:", *makePtr())

	// 9. 双指针
	a := 10
	pa := &a
	ppa := &pa
	fmt.Printf("a=%d, *pa=%d, **ppa=%d\n", a, *pa, **ppa)
}
