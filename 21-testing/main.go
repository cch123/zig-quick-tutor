package main

import "fmt"

// ============================================================
// 被测代码
// ============================================================

// Stack 泛型栈（Go 1.18+）
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
	val := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return val, true
}

func (s *Stack[T]) Peek() (T, bool) {
	if len(s.items) == 0 {
		var zero T
		return zero, false
	}
	return s.items[len(s.items)-1], true
}

func (s *Stack[T]) Size() int {
	return len(s.items)
}

// 数学函数
func Add(a, b int) int {
	return a + b
}

func Factorial(n uint) uint64 {
	if n == 0 {
		return 1
	}
	result := uint64(1)
	for i := uint(1); i <= n; i++ {
		result *= uint64(i)
	}
	return result
}

func Fibonacci(n uint) uint64 {
	if n <= 1 {
		return uint64(n)
	}
	a, b := uint64(0), uint64(1)
	for i := uint(2); i <= n; i++ {
		a, b = b, a+b
	}
	return b
}

type MathError struct {
	msg string
}

func (e *MathError) Error() string {
	return e.msg
}

func SafeDivide(a, b int) (int, error) {
	if b == 0 {
		return 0, &MathError{msg: "division by zero"}
	}
	return a / b, nil
}

func main() {
	fmt.Println("这是正常的 main 函数。")
	fmt.Println("运行 `go test -v` 来执行测试。")
}
