package main

import "fmt"

func main() {
	// 整数类型
	var a int8 = -128
	var b uint8 = 255
	var c int32 = 42
	var d uint64 = 18_446_744_073_709_551_615

	fmt.Println("int8:", a)
	fmt.Println("uint8:", b)
	fmt.Println("int32:", c)
	fmt.Println("uint64:", d)

	// 浮点类型
	var f1 float32 = 3.14
	var f2 float64 = 2.718281828
	fmt.Printf("float32: %.2f\n", f1)
	fmt.Printf("float64: %.9f\n", f2)

	// 布尔类型
	yes := true
	no := false
	fmt.Println("bool:", yes, no)

	// 字符 (rune = int32)
	var ch rune = 'A'
	fmt.Printf("char: %c\n", ch)
	fmt.Printf("char as int: %d\n", ch)

	// untyped constant: 编译期常量，精度不受限
	const big = 1_000_000_000_000_000
	var result uint64 = big * 2
	fmt.Println("untyped const result:", result)

	// 类型推断
	inferred := int32(100)
	fmt.Println("inferred:", inferred)
}
