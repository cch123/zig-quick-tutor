# 第十三章：可选类型（Optionals）

在编程中，"值可能不存在"是一个常见的场景。三种语言对此有截然不同的处理方式。

## Zig 的方式

Zig 使用 `?T` 表示可选类型，这是一个编译期已知的类型。可选值要么是 `null`，要么是一个有效的 `T` 值。

- `orelse`：提供默认值
- `if (opt) |val|`：安全解包
- `.?`：强制解包（如果是 null 会 panic）

Zig 的设计哲学是：**让 null 的处理显式化**，避免 C 语言中常见的空指针解引用问题。

## Go 的方式

Go 没有专门的可选类型。它依赖：

- 指针类型的 `nil`
- 零值（zero value）语义
- `(value, ok)` 模式（如 map 查找、类型断言）

这种方式简单直接，但也意味着编译器无法帮你检查是否遗漏了 nil 检查。

## Rust 的方式

Rust 使用 `Option<T>` 枚举，它是语言核心类型之一：

- `Some(T)` 表示有值
- `None` 表示无值
- 提供丰富的组合子：`map`、`and_then`、`unwrap_or` 等
- `?` 运算符可以提前返回 `None`

Rust 完全没有 null，所有"可能缺失"的值都必须用 `Option<T>` 表达。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 可选类型 | `?T` | 无（用指针/零值） | `Option<T>` |
| 空值表示 | `null` | `nil` / 零值 | `None` |
| 安全解包 | `if (opt) \|val\|` | `if v != nil` | `if let Some(v)` |
| 默认值 | `orelse` | 手动 `if` | `unwrap_or` |
| 强制解包 | `.?`（panic） | 直接用（可能 panic） | `unwrap()`（panic） |
| 链式操作 | 不支持 | 不支持 | `map`/`and_then` |
| 编译期安全 | 是 | 否 | 是 |

## 运行方式

```bash
# Zig
zig run main.zig

# Go
go run main.go

# Rust
rustc main.rs -o main_rs && ./main_rs
```
