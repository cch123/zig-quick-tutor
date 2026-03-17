package main

import "testing"

// ============================================================
// 测试 1: 基本测试
// ============================================================

func TestAdd(t *testing.T) {
	result := Add(1, 2)
	if result != 3 {
		t.Errorf("Add(1, 2) = %d; want 3", result)
	}
}

// ============================================================
// 测试 2: 表驱动测试（Go 的最佳实践）
// ============================================================

func TestFactorial(t *testing.T) {
	tests := []struct {
		name     string
		input    uint
		expected uint64
	}{
		{"zero", 0, 1},
		{"one", 1, 1},
		{"five", 5, 120},
		{"ten", 10, 3628800},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			result := Factorial(tc.input)
			if result != tc.expected {
				t.Errorf("Factorial(%d) = %d; want %d",
					tc.input, result, tc.expected)
			}
		})
	}
}

func TestFibonacci(t *testing.T) {
	tests := []struct {
		input    uint
		expected uint64
	}{
		{0, 0},
		{1, 1},
		{2, 1},
		{10, 55},
		{20, 6765},
	}

	for _, tc := range tests {
		result := Fibonacci(tc.input)
		if result != tc.expected {
			t.Errorf("Fibonacci(%d) = %d; want %d",
				tc.input, result, tc.expected)
		}
	}
}

// ============================================================
// 测试 3: Stack 测试
// ============================================================

func TestStack(t *testing.T) {
	s := &Stack[int]{}

	// 空栈 Pop
	_, ok := s.Pop()
	if ok {
		t.Error("Pop on empty stack should return false")
	}

	s.Push(1)
	s.Push(2)
	s.Push(3)

	if s.Size() != 3 {
		t.Errorf("Size() = %d; want 3", s.Size())
	}

	val, ok := s.Peek()
	if !ok || val != 3 {
		t.Errorf("Peek() = %d, %v; want 3, true", val, ok)
	}

	val, ok = s.Pop()
	if !ok || val != 3 {
		t.Errorf("Pop() = %d, %v; want 3, true", val, ok)
	}

	val, ok = s.Pop()
	if !ok || val != 2 {
		t.Errorf("Pop() = %d, %v; want 2, true", val, ok)
	}
}

// ============================================================
// 测试 4: 错误测试
// ============================================================

func TestSafeDivide(t *testing.T) {
	// 正常情况
	result, err := SafeDivide(10, 2)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != 5 {
		t.Errorf("SafeDivide(10, 2) = %d; want 5", result)
	}

	// 除以零
	_, err = SafeDivide(10, 0)
	if err == nil {
		t.Fatal("expected error for division by zero")
	}
}

// ============================================================
// 基准测试
// ============================================================

func BenchmarkFibonacci(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Fibonacci(20)
	}
}
