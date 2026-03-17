# 12 - 联合体 (Unions)

联合体（tagged union / sum type）是函数式编程中的核心概念，在系统编程中也至关重要。

## 核心概念

### Zig
Tagged union 是 Zig 的核心特性之一。它结合了枚举标记和数据存储，可以在 switch 中安全地解构。Zig 还支持 `extern union`（C 兼容的裸联合体）。

### Go
Go 没有联合体类型。通常用接口（interface）或包含多个字段的结构体来模拟类似功能。这种模拟不如真正的联合体类型安全。

### Rust
Rust 的枚举本身就是 tagged union。每个变体可以携带不同类型和数量的数据。`Option<T>` 和 `Result<T, E>` 就是 tagged union 的典型应用。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| Tagged union | `union(enum)` | 无（用 interface 模拟） | `enum` 自带 |
| 裸联合体 | `union` / `extern union` | 无 | `union`（unsafe） |
| Switch 解构 | 支持，捕获载荷 | 类型断言 switch | `match` 解构 |
| 穷尽性检查 | switch 必须穷尽 | 无保证 | match 必须穷尽 |
| C 互操作 | `extern union` | `cgo` + C 联合体 | `#[repr(C)]` union |
| 方法 | 支持 | N/A | 支持 |
| 内存大小 | 最大变体 + tag | N/A | 最大变体 + tag |
| 空值表示 | `?T`（可选类型） | `nil` | `Option<T>` |

## 要点

1. **Zig 的 tagged union 和枚举是分开的概念**：枚举只是标记，tagged union 是标记 + 数据。
2. **Go 缺少 tagged union**：这是 Go 类型系统的一个明显不足，接口模拟的方式没有穷尽性检查。
3. **Rust 的 enum 就是 tagged union**：这是 Rust 最强大的特性之一，`Option` 和 `Result` 无处不在。
4. **安全性**：Zig 和 Rust 的 tagged union 都有编译期穷尽性检查，避免遗漏分支。
5. **Zig 的可选类型 `?T`**：本质上是一个 tagged union `union(enum) { value: T, null }`。
