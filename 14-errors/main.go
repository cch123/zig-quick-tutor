package main

import (
	"errors"
	"fmt"
)

// 1. 定义哨兵错误
var (
	ErrNotFound         = errors.New("not found")
	ErrPermissionDenied = errors.New("permission denied")
	ErrDivisionByZero   = errors.New("division by zero")
)

// 2. 自定义错误类型
type FileError struct {
	Path    string
	Message string
}

func (e *FileError) Error() string {
	return fmt.Sprintf("file error: %s: %s", e.Path, e.Message)
}

// 3. (T, error) 模式
func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, ErrDivisionByZero
	}
	return a / b, nil
}

// 4. 错误包装
func readConfig(path string) (string, error) {
	if path == "missing.txt" {
		return "", fmt.Errorf("readConfig: %w", ErrNotFound)
	}
	if path == "secret.txt" {
		return "", &FileError{Path: path, Message: "permission denied"}
	}
	return "config_data", nil
}

// 5. 错误传播 - Go 的 if err != nil 模式
func loadAndProcess(path string) (string, error) {
	data, err := readConfig(path)
	if err != nil {
		return "", fmt.Errorf("loadAndProcess: %w", err)
	}
	_ = data
	return "processed_data", nil
}

func main() {
	// 1. 基本错误处理
	result, err := divide(10, 3)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Printf("10 / 3 = %.2f\n", result)

	// 2. 处理除零错误
	_, err = divide(10, 0)
	if err != nil {
		fmt.Println("10 / 0 error:", err)
	}

	// 3. errors.Is - 判断错误链中是否包含特定错误
	_, err = loadAndProcess("missing.txt")
	if errors.Is(err, ErrNotFound) {
		fmt.Println("File not found (detected via errors.Is)")
	}

	// 4. errors.As - 提取特定类型的错误
	_, err = readConfig("secret.txt")
	var fileErr *FileError
	if errors.As(err, &fileErr) {
		fmt.Printf("FileError on path %q: %s\n", fileErr.Path, fileErr.Message)
	}

	// 5. 成功路径
	config, err := readConfig("normal.txt")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Good config:", config)

	// 注意：Go 允许忽略错误返回值，编译器不会警告
	// result2, _ := divide(10, 0)  // 忽略错误，这是合法但危险的
}
