# 01 - Hello World

## 概述

每门语言的第一个程序。通过最简单的 Hello World，我们可以看到三种语言在程序入口、打印函数和字符串处理上的差异。

## 编译与运行

```bash
# Zig
zig run hello.zig

# Go
go run hello.go

# Rust
rustc hello.rs -o hello && ./hello
```

## 代码解析

### Zig

```zig
const std = @import("std");

pub fn main() void {
    std.debug.print("Hello, World!\n", .{});
}
```

- `@import("std")` 导入标准库，赋值给编译期常量 `std`
- `pub fn main() void` — 入口函数必须是 `pub`，返回类型显式写为 `void`
- `std.debug.print` 接受一个格式字符串和一个匿名结构体 `.{}`（类似 printf 的参数列表）
- 字符串字面量的类型是 `*const [N:0]u8`（以 0 结尾的字节数组指针）

### Go

```go
func main() {
    fmt.Println("Hello, World!")
}
```

- 作为 Go 开发者你已经很熟悉了
- `fmt.Println` 自动追加换行符
- 字符串字面量的类型是 `string`（不可变的 UTF-8 字节序列）

### Rust

```rust
fn main() {
    println!("Hello, World!");
}
```

- `println!` 是宏（注意 `!`），不是函数
- `fn main()` 不需要写返回类型（默认返回 `()`）
- 字符串字面量的类型是 `&str`（字符串切片引用）

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 入口函数 | `pub fn main() void` | `func main()` | `fn main()` |
| 打印函数 | `std.debug.print(fmt, .{args})` | `fmt.Println(args)` | `println!(fmt, args)` |
| 字符串类型 | `*const [N:0]u8` | `string` | `&str` |
| 自动换行 | 否，需手动 `\n` | `Println` 自动换行 | `println!` 自动换行 |
| 导入方式 | `@import("std")` | `import "fmt"` | 标准库自动可用 |
| 编译命令 | `zig build-exe` / `zig run` | `go build` / `go run` | `rustc` / `cargo build` |

## 要点

1. **Zig 没有隐式行为**：不会自动换行，不会自动导入任何东西。这是 Zig 的设计哲学 —— "没有隐藏的控制流，没有隐藏的分配"。
2. **Zig 的 `print` 使用编译期格式化**：`.{}` 是匿名结构体语法，即使打印简单字符串也需要传入空参数。
3. 对比 Go 的 `fmt.Println`，Zig 的打印更接近 C 的 `printf` 风格，但类型安全。
