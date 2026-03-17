package main

import (
	"errors"
	"fmt"
)

// --- 基本函数 ---
func add(a, b int32) int32 {
	return a + b
}

// --- 多返回值 ---
func swap(a, b int32) (int32, int32) {
	return b, a
}

// --- 错误返回 ---
var ErrDivisionByZero = errors.New("division by zero")

func safeDivide(a, b int32) (int32, error) {
	if b == 0 {
		return 0, ErrDivisionByZero
	}
	return a / b, nil
}

// --- 函数作为一等公民 ---
func applyOp(a, b int32, op func(int32, int32) int32) int32 {
	return op(a, b)
}

func multiply(a, b int32) int32 {
	return a * b
}

// --- Go 没有 comptime，但有泛型 (Go 1.18+) ---
func maxOf[T int32 | int64 | float32 | float64](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	// 基本函数调用
	fmt.Println("add(3, 4) =", add(3, 4))

	// 多返回值
	x, y := swap(10, 20)
	fmt.Printf("swap(10, 20) = %d, %d\n", x, y)

	// 错误处理
	val, err := safeDivide(10, 3)
	if err != nil {
		fmt.Println("error:", err)
	} else {
		fmt.Println("10 / 3 =", val)
	}

	_, err = safeDivide(10, 0)
	if err != nil {
		fmt.Println("10 / 0 = error:", err)
	}

	// 函数指针
	fmt.Println("applyOp(3, 4, multiply) =", applyOp(3, 4, multiply))

	// 匿名函数
	double := func(x int32) int32 { return x * 2 }
	fmt.Println("double(5) =", double(5))

	// 泛型
	fmt.Println("maxOf(3, 7) =", maxOf(int32(3), int32(7)))
	fmt.Printf("maxOf(1.5, 2.5) = %.1f\n", maxOf(1.5, 2.5))
}
