package main

import "fmt"

func getOptionalValue(check bool) *int {
	if check {
		v := 42
		return &v
	}
	return nil
}

func main() {
	// === 基本 if/else ===
	x := 10

	if x > 5 {
		fmt.Println("x 大于 5")
	} else if x > 0 {
		fmt.Println("x 大于 0 但不大于 5")
	} else {
		fmt.Println("x 小于等于 0")
	}

	// === Go 没有三元运算符，必须用 if/else 语句 ===
	var y int
	if x > 5 {
		y = 100
	} else {
		y = 200
	}
	fmt.Printf("y = %d\n", y)

	// === 同样，初始化变量也需要 if 语句 ===
	var label string
	if x%2 == 0 {
		label = "even"
	} else {
		label = "odd"
	}
	fmt.Printf("x is %s\n", label)

	// === Go 的 nil 检查（对比 Zig 的 optional 捕获） ===
	maybeVal := getOptionalValue(true)
	if maybeVal != nil {
		fmt.Printf("捕获到值: %d\n", *maybeVal)
	} else {
		fmt.Println("值为 nil")
	}

	noVal := getOptionalValue(false)
	if noVal != nil {
		fmt.Printf("捕获到值: %d\n", *noVal)
	} else {
		fmt.Println("值为 nil")
	}

	// === Go 特色：if 中声明变量 ===
	if val := getOptionalValue(true); val != nil {
		fmt.Printf("if 中声明并使用: %d\n", *val)
	}
	// val 在这里已经不可访问

	// === bool 运算（Go 用 && || ） ===
	a := true
	b := false
	if a && !b {
		fmt.Println("a 为 true 且 b 为 false")
	}
	if a || b {
		fmt.Println("a 或 b 至少一个为 true")
	}
}
