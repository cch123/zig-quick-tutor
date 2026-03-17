# 07 - Loops 循环

## 概述

三种语言的循环设计差异很大。Go 只有 `for` 一个循环关键字；Rust 有 `loop`、`while`、`for`；Zig 有 `while` 和 `for`，但 `for` 仅用于迭代切片和范围，没有传统 C 风格 for 循环。

## 核心差异

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 循环关键字 | `while`, `for` | `for`（唯一） | `loop`, `while`, `for` |
| C 风格 for | 不支持 | 支持 | 不支持 |
| 无限循环 | `while (true)` | `for { }` | `loop { }` |
| 范围迭代 | `for (0..n)` | `for i := 0; i < n; i++` | `for i in 0..n` |
| 切片迭代 | `for (slice) \|item\|` | `for i, v := range slice` | `for item in &slice` |
| break 返回值 | 支持 | 不支持 | 支持（仅 loop） |
| 标签循环 | 支持 | 支持 | 支持 |
| continue 后执行 | while 支持 `|i| : (i += 1)` | for 的 post 语句 | 不支持 |

## Zig 特色

1. **while 带 continue 表达式**：`while (cond) : (update) { }` 中 update 部分在每次迭代末尾（包括 continue 时）执行。
2. **for 仅用于迭代**：不是通用循环，只迭代切片和范围。
3. **break 可以返回值**：`break value` 让 while 成为表达式。
4. **同时迭代多个切片**：`for (a, b) |x, y|`。

## Go 特色

1. **只有 for**：`for` 是唯一的循环关键字，但可以当 while 和无限循环用。
2. **range 迭代**：`for i, v := range collection` 是 Go 的惯用迭代方式。
3. **简洁统一**：一个关键字覆盖所有场景。

## Rust 特色

1. **loop 是无限循环**：专门的关键字，比 `while true` 更清晰。
2. **loop 可以 break 返回值**：`let x = loop { break 42; };`。
3. **for..in 迭代器**：基于迭代器 trait，非常灵活。
4. **标签用单引号**：`'outer: loop { }`。

## 运行方式

```bash
# Zig
zig run main.zig

# Go
go run main.go

# Rust
rustc main.rs -o main_rs && ./main_rs
```
