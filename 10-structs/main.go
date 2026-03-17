package main

import (
	"fmt"
	"math"
	"unsafe"
)

// === 基本结构体 ===
type Point struct {
	X, Y float64
}

// 方法：值接收者
func (p Point) DistanceTo(other Point) float64 {
	dx := p.X - other.X
	dy := p.Y - other.Y
	return math.Sqrt(dx*dx + dy*dy)
}

func (p Point) String() string {
	return fmt.Sprintf("Point(%.1f, %.1f)", p.X, p.Y)
}

// 构造函数惯用法
func NewPoint(x, y float64) Point {
	return Point{X: x, Y: y}
}

// === 带默认值的结构体（Go 用零值） ===
type Config struct {
	Host  string
	Port  int
	Debug bool
}

func NewConfig() Config {
	return Config{
		Host: "localhost",
		Port: 8080,
	}
}

func (c Config) String() string {
	return fmt.Sprintf("Config{host=%s, port=%d, debug=%v}", c.Host, c.Port, c.Debug)
}

// === 可变方法：指针接收者 ===
type Counter struct {
	count int
}

func (c *Counter) Increment() {
	c.count++
}

func (c Counter) GetCount() int {
	return c.count
}

// === 嵌入（组合） ===
type Animal struct {
	Name string
}

func (a Animal) Speak() string {
	return a.Name + " makes a sound"
}

type Dog struct {
	Animal // 匿名嵌入
	Breed  string
}

// === 泛型结构体 (Go 1.18+) ===
type Pair[T any] struct {
	First  T
	Second T
}

func NewPair[T any](first, second T) Pair[T] {
	return Pair[T]{First: first, Second: second}
}

func (p Pair[T]) Swap() Pair[T] {
	return Pair[T]{First: p.Second, Second: p.First}
}

func main() {
	// === 基本使用 ===
	p1 := NewPoint(0, 0)
	p2 := Point{X: 3, Y: 4}
	fmt.Printf("%v 到 %v 的距离: %.2f\n", p1, p2, p1.DistanceTo(p2))

	// === 零值 ===
	var zeroCfg Config // 所有字段都是零值
	fmt.Println("零值配置:", zeroCfg)

	defaultCfg := NewConfig()
	customCfg := Config{Host: "example.com", Port: 9090, Debug: true}
	fmt.Println("默认配置:", defaultCfg)
	fmt.Println("自定义配置:", customCfg)

	// === 指针接收者 ===
	counter := Counter{}
	counter.Increment()
	counter.Increment()
	counter.Increment()
	fmt.Println("计数器:", counter.GetCount())

	// === 匿名结构体 ===
	anon := struct {
		Name    string
		Version int
	}{
		Name:    "Go",
		Version: 21,
	}
	fmt.Printf("匿名结构体: name=%s, version=%d\n", anon.Name, anon.Version)

	// === 嵌入（组合） ===
	dog := Dog{
		Animal: Animal{Name: "Buddy"},
		Breed:  "Labrador",
	}
	fmt.Println("嵌入:", dog.Speak()) // 直接调用嵌入类型的方法
	fmt.Println("品种:", dog.Breed)

	// === 泛型结构体 ===
	intPair := NewPair(10, 20)
	swapped := intPair.Swap()
	fmt.Printf("原始: (%d, %d)\n", intPair.First, intPair.Second)
	fmt.Printf("交换: (%d, %d)\n", swapped.First, swapped.Second)

	// === 结构体大小 ===
	fmt.Println("\n--- 内存布局 ---")
	fmt.Println("Point 大小:", unsafe.Sizeof(Point{}), "字节")
	fmt.Println("Config 大小:", unsafe.Sizeof(Config{}), "字节")
	fmt.Println("Counter 大小:", unsafe.Sizeof(Counter{}), "字节")
}
