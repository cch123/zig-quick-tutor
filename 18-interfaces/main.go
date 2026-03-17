package main

import (
	"fmt"
	"math"
)

// 1. 接口定义 —— 隐式满足，无需声明 implements
type Shape interface {
	Area() float64
	Describe() string
}

// 2. Circle 隐式实现 Shape 接口
type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c Circle) Describe() string {
	return "I am a circle"
}

// 3. Rectangle 隐式实现 Shape 接口
type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle) Describe() string {
	return "I am a rectangle"
}

// 4. 使用接口作为参数 —— 动态分派
func printArea(s Shape) {
	fmt.Printf("%s area = %.2f\n", s.Describe(), s.Area())
}

// 5. 接口组合
type Drawable interface {
	Draw()
}

type DrawableShape interface {
	Shape
	Drawable
}

// 6. 空接口 any / interface{} —— 可以持有任何值
func printAnything(v any) {
	fmt.Printf("value = %v (type: %T)\n", v, v)
}

// 7. Stringer 接口 —— 类似其他语言的 toString()
type Star struct {
	Points int
}

func (s Star) Draw() {
	fmt.Printf("Drawing a %d-pointed star\n", s.Points)
}

func (s Star) String() string {
	return fmt.Sprintf("Star(%d)", s.Points)
}

type Line struct {
	Length float64
}

func (l Line) Draw() {
	fmt.Printf("Drawing a line of length %.1f\n", l.Length)
}

func main() {
	// 1. 接口多态 —— 不同类型放入同一个切片
	fmt.Println("=== interface polymorphism ===")
	shapes := []Shape{
		Circle{Radius: 5.0},
		Rectangle{Width: 3.0, Height: 4.0},
	}
	for _, s := range shapes {
		printArea(s)
	}

	// 2. 类型断言 —— 运行时获取具体类型
	fmt.Println("\n=== type assertion ===")
	var s Shape = Circle{Radius: 3.0}
	if c, ok := s.(Circle); ok {
		fmt.Printf("It's a circle with radius %.1f\n", c.Radius)
	}

	// 3. type switch —— 类型分支
	fmt.Println("\n=== type switch ===")
	for _, shape := range shapes {
		switch v := shape.(type) {
		case Circle:
			fmt.Printf("Circle: radius = %.1f\n", v.Radius)
		case Rectangle:
			fmt.Printf("Rectangle: %.1f x %.1f\n", v.Width, v.Height)
		}
	}

	// 4. 空接口
	fmt.Println("\n=== any (interface{}) ===")
	printAnything(42)
	printAnything("hello")
	printAnything(3.14)

	// 5. Drawable 接口
	fmt.Println("\n=== Drawable interface ===")
	drawables := []Drawable{
		Star{Points: 5},
		Line{Length: 10.0},
	}
	for _, d := range drawables {
		d.Draw()
	}

	// Go 的接口是隐式满足的 —— 结构体不需要声明它实现了哪些接口
	// 这使得代码解耦更加彻底，但也导致难以一眼看出类型实现了哪些接口
}
