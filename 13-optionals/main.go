package main

import "fmt"

// Go 没有可选类型，用指针 + nil 或 (value, ok) 模式代替

// 用指针返回"可选"值
func findEven(items []int) *int {
	for _, item := range items {
		if item%2 == 0 {
			result := item
			return &result
		}
	}
	return nil
}

// 用 (value, ok) 模式
func findEvenOk(items []int) (int, bool) {
	for _, item := range items {
		if item%2 == 0 {
			return item, true
		}
	}
	return 0, false
}

// 用指针参数模拟可选参数
func greet(name *string) {
	if name != nil {
		fmt.Printf("Hello, %s!\n", *name)
	} else {
		fmt.Println("Hello, stranger!")
	}
}

func main() {
	// 1. 指针可以为 nil（类似可选）
	var ptr *int = nil
	fmt.Println("ptr is nil:", ptr == nil) // true

	val := 42
	ptr = &val
	fmt.Println("ptr value:", *ptr) // 42

	// 2. 零值 - Go 的每种类型都有零值
	var num int     // 0
	var str string  // ""
	var b bool      // false
	var sl []int    // nil
	fmt.Printf("Zero values: num=%d, str=%q, b=%v, sl=%v\n", num, str, b, sl)

	// 3. Map 查找的 (value, ok) 模式
	m := map[string]int{"a": 1, "b": 2}
	if v, ok := m["a"]; ok {
		fmt.Println("Found:", v)
	}
	if _, ok := m["z"]; !ok {
		fmt.Println("Key 'z' not found")
	}

	// 4. 返回指针的函数
	items := []int{1, 3, 5, 4, 7}
	if even := findEven(items); even != nil {
		fmt.Println("First even:", *even)
	}

	odds := []int{1, 3, 5, 7}
	if even := findEven(odds); even != nil {
		fmt.Println("First even:", *even)
	} else {
		fmt.Println("No even number found")
	}

	// 5. (value, ok) 模式
	if v, ok := findEvenOk(items); ok {
		fmt.Println("First even (ok pattern):", v)
	}

	// 6. 指针参数模拟可选
	name := "Go"
	greet(&name)
	greet(nil)

	// 注意：nil 解引用会 panic，编译器不会帮你检查！
	// var bad *int
	// fmt.Println(*bad) // panic: runtime error
}
