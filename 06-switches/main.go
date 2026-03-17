package main

import (
	"fmt"
	"math"
)

// Go 用接口+类型断言模拟 tagged union
type Shape interface {
	Area() float64
}

type Circle struct{ Radius float64 }
type Rectangle struct{ W, H float64 }
type Triangle struct{ Base, Height float64 }

func (c Circle) Area() float64    { return math.Pi * c.Radius * c.Radius }
func (r Rectangle) Area() float64 { return r.W * r.H }
func (t Triangle) Area() float64  { return t.Base * t.Height / 2 }

func main() {
	// === 基本 switch ===
	x := 3

	// Go 的 switch 不是表达式，不能赋值
	var label string
	switch x {
	case 1:
		label = "one"
	case 2:
		label = "two"
	case 3:
		label = "three"
	case 4, 5: // 多值匹配
		label = "four or five"
	default:
		label = "other"
	}
	// 注意：Go 不支持范围匹配（如 6...10），需要用 default 或无条件 switch
	fmt.Printf("x = %d, label = %s\n", x, label)

	// === 无条件 switch（等同于 if/else if） ===
	grade := 85
	switch {
	case grade >= 90:
		fmt.Println("优秀 (A)")
	case grade >= 80:
		fmt.Println("良好 (B)")
	case grade >= 70:
		fmt.Println("中等 (C)")
	case grade >= 60:
		fmt.Println("及格 (D)")
	default:
		fmt.Println("不及格 (F)")
	}

	// === type switch：Go 的特色 ===
	shapes := []Shape{
		Circle{Radius: 5.0},
		Rectangle{W: 4.0, H: 6.0},
		Triangle{Base: 3.0, Height: 8.0},
	}

	for _, s := range shapes {
		var name string
		switch s.(type) {
		case Circle:
			name = "圆形"
		case Rectangle:
			name = "矩形"
		case Triangle:
			name = "三角形"
		}
		fmt.Printf("%s 的面积 = %.2f\n", name, s.Area())
	}

	// === fallthrough 演示 ===
	val := 1
	switch val {
	case 1:
		fmt.Println("matched 1")
		fallthrough // 显式穿透到下一个 case
	case 2:
		fmt.Println("matched 2 (or fell through)")
	case 3:
		fmt.Println("matched 3")
	}

	// === switch 中声明变量 ===
	switch flag := true; flag {
	case true:
		fmt.Println("开关状态: 开")
	case false:
		fmt.Println("开关状态: 关")
	}
}
