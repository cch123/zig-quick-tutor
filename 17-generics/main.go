package main

import (
	"cmp"
	"fmt"
)

// 1. 泛型函数 —— 类型参数 + 约束
func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

// 2. 泛型容器：栈
type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(value T) {
	s.items = append(s.items, value)
}

func (s *Stack[T]) Pop() (T, bool) {
	if len(s.items) == 0 {
		var zero T
		return zero, false
	}
	last := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return last, true
}

func (s *Stack[T]) Peek() (T, bool) {
	if len(s.items) == 0 {
		var zero T
		return zero, false
	}
	return s.items[len(s.items)-1], true
}

// 3. 自定义约束 —— 接口作为类型约束
type Number interface {
	~int | ~int32 | ~int64 | ~float32 | ~float64
}

func Double[T Number](x T) T {
	return x * 2
}

// 4. 泛型 Map 函数
func Map[T any, U any](slice []T, f func(T) U) []U {
	result := make([]U, len(slice))
	for i, v := range slice {
		result[i] = f(v)
	}
	return result
}

func main() {
	// 1. 泛型 Max
	fmt.Println("Max(int):", Max(10, 20))
	fmt.Printf("Max(float64): %.1f\n", Max(3.14, 2.71))

	// 2. 泛型 Double
	fmt.Println("Double(int):", Double(21))
	fmt.Printf("Double(float64): %.1f\n", Double(1.5))

	// 3. 泛型栈
	intStack := Stack[int]{}
	intStack.Push(10)
	intStack.Push(20)
	intStack.Push(30)
	if v, ok := intStack.Peek(); ok {
		fmt.Println("peek:", v)
	}
	if v, ok := intStack.Pop(); ok {
		fmt.Println("pop:", v)
	}
	if v, ok := intStack.Pop(); ok {
		fmt.Println("pop:", v)
	}

	strStack := Stack[string]{}
	strStack.Push("hello")
	strStack.Push("world")
	if v, ok := strStack.Pop(); ok {
		fmt.Println("string pop:", v)
	}

	// 4. 泛型高阶函数
	nums := []int{1, 2, 3, 4, 5}
	squares := Map(nums, func(x int) int { return x * x })
	fmt.Println("squares:", squares)

	strs := Map(nums, func(x int) string { return fmt.Sprintf("item_%d", x) })
	fmt.Println("mapped to strings:", strs)

	// 注意：Go 没有编译期计算，所有泛型逻辑在运行时通过字典传递实现
	// Go 的泛型设计偏保守，不支持方法上的类型参数等高级特性
}
