package main

import "fmt"

// 包级常量
const PI = 3.141592653589793

func main() {
	// --- 变量声明 ---
	// var 声明
	var x int32 = 10
	x += 5
	fmt.Println("var x =", x)

	// 短变量声明
	y := 42
	fmt.Println(":= y =", y)

	// 类型推断
	z := int32(100)
	fmt.Println("inferred z =", z)

	// --- 常量 ---
	const compileTimeVal = 1 * 2 * 3 * 4 * 5 // 120
	fmt.Println("const 5! =", compileTimeVal)

	fmt.Printf("PI = %.6f\n", PI)

	// --- Go 允许 Shadowing ---
	a := 10
	fmt.Println("a =", a)
	{
		a := 20 // 新的变量，遮蔽了外层的 a
		fmt.Println("shadowed a =", a)
	}
	fmt.Println("original a =", a) // 仍然是 10

	// --- 零值 ---
	// Go 的变量自动初始化为零值
	var i int     // 0
	var f float64 // 0.0
	var b bool    // false
	var s string  // ""
	fmt.Printf("zero values: %d %.1f %v %q\n", i, f, b, s)

	// --- 块作用域 ---
	{
		scoped := 123
		fmt.Println("scoped =", scoped)
	}
	// scoped 在这里不可见
}
