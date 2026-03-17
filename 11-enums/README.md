# 11 - 枚举 (Enums)

枚举用于表示一组有限的可能值。三种语言对枚举的支持差异巨大。

## 核心概念

### Zig
Zig 的枚举功能强大：可以指定底层整数类型、定义方法、声明为非穷尽（non-exhaustive）以支持未来扩展。枚举值可以与整数互相转换。

### Go
Go 没有真正的枚举类型，而是使用 `const` + `iota` 模式模拟。这种方式灵活但类型安全性较弱——任何同类型的整数都可以作为"枚举"值。

### Rust
Rust 的枚举是代数数据类型（ADT），每个变体可以携带不同类型的关联数据。这使得 Rust 枚举远比其他语言的枚举强大（类似于 Haskell 的 sum type）。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 枚举语法 | `enum` 关键字 | `const` + `iota` | `enum` 关键字 |
| 类型安全 | 强 | 弱 | 强 |
| 关联数据 | 不支持（用 tagged union） | 不支持 | 支持 |
| 方法 | 支持 | 不直接支持 | 支持 |
| 底层类型控制 | `enum(u8)` 等 | 类型别名 | `#[repr(u8)]` 等 |
| 非穷尽 | `_` 标记 | N/A | `#[non_exhaustive]` |
| 穷尽性检查 | switch 必须穷尽 | 无 | match 必须穷尽 |
| 整数转换 | `@intFromEnum` / `@enumFromInt` | 直接转换 | `as` / `TryFrom` |

## 要点

1. **Go 的 "枚举" 只是常量**：没有编译器级别的穷尽性检查。
2. **Zig 的枚举和 tagged union 分工明确**：枚举用于简单标记，tagged union 用于关联数据（见第 12 章）。
3. **Rust 的枚举 = 枚举 + tagged union**：`Option<T>` 和 `Result<T, E>` 就是枚举的典型应用。
4. **穷尽性检查**：Zig 和 Rust 都要求 switch/match 覆盖所有可能值，这能在编译期捕获遗漏。
