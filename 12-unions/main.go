package main

import (
	"fmt"
	"math"
)

// === Go 没有联合体，用接口模拟 tagged union ===

// Shape 接口模拟 tagged union
type Shape interface {
	Area() float64
	Name() string
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 { return math.Pi * c.Radius * c.Radius }
func (c Circle) Name() string  { return "Circle" }

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 { return r.Width * r.Height }
func (r Rectangle) Name() string  { return "Rectangle" }

type Triangle struct {
	Base, Height float64
}

func (t Triangle) Area() float64 { return 0.5 * t.Base * t.Height }
func (t Triangle) Name() string  { return "Triangle" }

// === 另一种模拟方式：struct + type tag ===

type TokenType int

const (
	TokenNumber TokenType = iota
	TokenString
	TokenPlus
	TokenMinus
	TokenEOF
)

type Token struct {
	Type     TokenType
	NumVal   float64 // 仅 TokenNumber 时有意义
	StrVal   string  // 仅 TokenString 时有意义
}

func (t Token) String() string {
	switch t.Type {
	case TokenNumber:
		return fmt.Sprintf("Number(%g)", t.NumVal)
	case TokenString:
		return fmt.Sprintf("String(%q)", t.StrVal)
	case TokenPlus:
		return "Plus"
	case TokenMinus:
		return "Minus"
	case TokenEOF:
		return "EOF"
	default:
		return "Unknown"
	}
}

// === 用指针模拟可选值 ===
func findIndex(data string, target byte) *int {
	for i := 0; i < len(data); i++ {
		if data[i] == target {
			idx := i
			return &idx
		}
	}
	return nil
}

// === 用多返回值模拟 Result ===
func parseDigit(c byte) (uint8, error) {
	if c >= '0' && c <= '9' {
		return c - '0', nil
	}
	return 0, fmt.Errorf("invalid character: %c", c)
}

func main() {
	// === 接口模拟 tagged union ===
	shapes := []Shape{
		Circle{Radius: 5.0},
		Rectangle{Width: 4.0, Height: 6.0},
		Triangle{Base: 3.0, Height: 8.0},
	}

	fmt.Println("=== 形状面积 ===")
	for _, shape := range shapes {
		fmt.Printf("  %s: %.2f\n", shape.Name(), shape.Area())
	}

	// === 类型断言 switch（类似 match） ===
	fmt.Println("\n=== 类型断言 ===")
	for _, shape := range shapes {
		switch s := shape.(type) {
		case Circle:
			fmt.Printf("  圆的半径: %g\n", s.Radius)
		case Rectangle:
			fmt.Printf("  矩形: %gx%g\n", s.Width, s.Height)
		case Triangle:
			fmt.Printf("  三角形: base=%g, h=%g\n", s.Base, s.Height)
		}
		// 注意：没有穷尽性检查！如果添加新类型但忘记处理，编译器不会报错
	}

	// === struct + tag 模拟 ===
	tokens := []Token{
		{Type: TokenNumber, NumVal: 42},
		{Type: TokenPlus},
		{Type: TokenNumber, NumVal: 8},
		{Type: TokenEOF},
	}

	fmt.Println("\n=== 词法标记 ===")
	for _, token := range tokens {
		fmt.Printf("  %s\n", token)
	}

	// === nil 作为可选值 ===
	fmt.Println("\n=== 可选值（nil） ===")
	data := "Hello, World!"
	if idx := findIndex(data, 'W'); idx != nil {
		fmt.Printf("找到 'W' 在位置: %d\n", *idx)
	} else {
		fmt.Println("未找到")
	}

	if idx := findIndex(data, 'Z'); idx != nil {
		fmt.Printf("找到 'Z' 在位置: %d\n", *idx)
	} else {
		fmt.Println("未找到 'Z'")
	}

	// === 多返回值模拟 Result ===
	fmt.Println("\n=== 错误处理 ===")
	if digit, err := parseDigit('5'); err == nil {
		fmt.Printf("解析 '5': %d\n", digit)
	} else {
		fmt.Printf("错误: %v\n", err)
	}

	if digit, err := parseDigit('x'); err == nil {
		fmt.Printf("解析 'x': %d\n", digit)
	} else {
		fmt.Printf("解析 'x' 错误: %v\n", err)
	}

	// === 接口的局限性 ===
	fmt.Println("\n=== Go 模拟 tagged union 的问题 ===")
	fmt.Println("1. 没有穷尽性检查：添加新类型时编译器不提醒")
	fmt.Println("2. 接口可以被任意类型实现：无法限制变体集合")
	fmt.Println("3. struct+tag 方式浪费内存：每个 token 都存所有字段")
	fmt.Println("4. struct+tag 方式无类型安全：可以访问错误的字段")
}
