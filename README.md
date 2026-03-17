# Zig by Example (对比 Go & Rust)

一套通过实例学习 Zig 的教程，每个示例同时展示 Zig、Go、Rust 三种语言的写法，帮助有 Go/Rust 经验的开发者快速上手 Zig。

内容是用 claude 参考 Go by Example 生成的。

## 目录

| # | 主题 | 说明 |
|---|------|------|
| 01 | [Hello World](01-hello-world/) | 第一个程序 |
| 02 | [Values & Types](02-values-types/) | 基本类型与值 |
| 03 | [Variables](03-variables/) | 变量声明与常量 |
| 04 | [Functions](04-functions/) | 函数定义与调用 |
| 05 | [If/Else](05-if-else/) | 条件判断 |
| 06 | [Switch/Match](06-switch/) | 多分支匹配 |
| 07 | [Loops](07-loops/) | 循环 (for/while) |
| 08 | [Arrays & Slices](08-arrays-slices/) | 数组与切片 |
| 09 | [Strings](09-strings/) | 字符串处理 |
| 10 | [Structs](10-structs/) | 结构体 |
| 11 | [Enums](11-enums/) | 枚举类型 |
| 12 | [Unions](12-unions/) | 联合类型 |
| 13 | [Optionals](13-optionals/) | 可选类型与空值处理 |
| 14 | [Error Handling](14-errors/) | 错误处理 |
| 15 | [Pointers](15-pointers/) | 指针 |
| 16 | [Defer](16-defer/) | 延迟执行 |
| 17 | [Generics (comptime)](17-generics/) | 泛型与编译期计算 |
| 18 | [Interfaces & Traits](18-interfaces/) | 接口/Trait/鸭子类型 |
| 19 | [Memory Management](19-memory/) | 内存管理 |
| 20 | [Concurrency](20-concurrency/) | 并发编程 |
| 21 | [Testing](21-testing/) | 测试 |
| 22 | [Build System](22-build-system/) | 构建系统 |

## 运行示例

```bash
# Zig
cd 01-hello-world && zig run hello.zig

# Go
cd 01-hello-world && go run hello.go

# Rust
cd 01-hello-world && rustc hello.rs -o hello && ./hello
```

## 环境要求

- Zig 0.14+
- Go 1.22+
- Rust 1.75+
