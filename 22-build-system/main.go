package main

import (
	"fmt"
	"runtime"
)

// ============================================================
// Go 构建系统演示
//
// Go 的构建系统设计哲学：简单。
//
// 常用命令:
//   go build              构建当前包
//   go run main.go        编译并运行
//   go test               运行测试
//   go mod init           初始化模块
//   go mod tidy           清理依赖
//   go generate           运行代码生成
//   go vet                静态分析
//
// go.mod 示例:
//   module example.com/myapp
//   go 1.22
//   require (
//       github.com/gin-gonic/gin v1.9.1
//   )
//
// 交叉编译:
//   GOOS=linux GOARCH=amd64 go build
//   GOOS=windows GOARCH=amd64 go build
//   GOOS=darwin GOARCH=arm64 go build
//
// 构建标签（条件编译）:
//   //go:build linux
//   //go:build !windows
// ============================================================

func main() {
	fmt.Println("=== Go 构建系统信息 ===")
	fmt.Println()

	// 运行时信息
	fmt.Printf("Go 版本: %s\n", runtime.Version())
	fmt.Printf("目标操作系统: %s\n", runtime.GOOS)
	fmt.Printf("目标架构: %s\n", runtime.GOARCH)
	fmt.Printf("编译器: %s\n", runtime.Compiler)
	fmt.Printf("CPU 核心数: %d\n", runtime.NumCPU())
	fmt.Printf("GOMAXPROCS: %d\n", runtime.GOMAXPROCS(0))
	fmt.Println()

	// 构建模式说明
	fmt.Println("=== Go 构建命令 ===")
	fmt.Println()
	fmt.Println("基本构建:")
	fmt.Println("  go build              # 构建当前目录")
	fmt.Println("  go build -o app       # 指定输出文件名")
	fmt.Println("  go build ./...        # 构建所有子包")
	fmt.Println()
	fmt.Println("优化与调试:")
	fmt.Println("  go build              # 默认包含调试信息")
	fmt.Println("  go build -ldflags '-s -w'  # 去除调试信息，减小体积")
	fmt.Println("  go build -race        # 启用竞态检测器")
	fmt.Println()
	fmt.Println("交叉编译:")
	fmt.Println("  GOOS=linux GOARCH=amd64 go build")
	fmt.Println("  GOOS=linux GOARCH=arm64 go build")
	fmt.Println("  GOOS=windows GOARCH=amd64 go build")
	fmt.Println()

	// 构建时注入变量（ldflags）
	fmt.Println("=== 构建时注入变量 ===")
	fmt.Println()
	fmt.Println("Go 可以通过 ldflags 在构建时注入变量:")
	fmt.Println("  go build -ldflags '-X main.Version=1.0.0 -X main.BuildTime=2024-01-01'")
	fmt.Println()
	fmt.Printf("当前 Version: %s\n", Version)
	fmt.Printf("当前 BuildTime: %s\n", BuildTime)
	fmt.Println()

	// CGo 说明
	fmt.Println("=== CGo (C 语言互操作) ===")
	fmt.Println()
	fmt.Println("Go 通过 CGo 调用 C 代码:")
	fmt.Println("  // #include <stdlib.h>")
	fmt.Println("  // #cgo LDFLAGS: -lsqlite3")
	fmt.Println("  import \"C\"")
	fmt.Println()
	fmt.Println("注意: CGo 会显著增加编译时间和二进制体积")
	fmt.Println("      交叉编译时 CGo 配置较复杂")
	fmt.Println("      (这正是 Zig 交叉编译的优势所在)")
}

// 可通过 ldflags 注入的变量
var (
	Version   = "dev"
	BuildTime = "unknown"
)
