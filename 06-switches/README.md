# 06 - Switch/Match 多路分支

## 概述

多路分支是条件判断的扩展。Go 使用 `switch`，Rust 使用 `match`，Zig 也使用 `switch`。三者的设计理念有明显不同，尤其在模式匹配能力上。

## 核心差异

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 关键字 | `switch` | `switch` | `match` |
| 是表达式 | 是 | 否 | 是 |
| 范围匹配 | `1...5` | 不支持 | `1..=5` |
| 多值匹配 | `1, 2, 3` | `1, 2, 3`（用逗号） | `1 \| 2 \| 3` |
| 穷尽性检查 | 是（需要 else） | 否 | 是 |
| 自动 fallthrough | 否 | 否（需显式 fallthrough） | 否 |
| 类型匹配 | 通过 tagged union | type switch | 通过 enum match |
| 捕获/绑定 | `\|val\|` 捕获 | 类型断言绑定 | 变量绑定 |

## Zig 特色

1. **switch 是表达式**：可以赋值给变量。
2. **范围匹配**：`1...5` 匹配 1 到 5（包含两端）。
3. **必须穷尽**：所有可能的值都必须被覆盖，否则需要 `else` 分支。
4. **tagged union 匹配**：Zig 用 tagged union 代替传统枚举，switch 可以匹配并捕获内部值。

## Go 特色

1. **默认不 fallthrough**：与 C 不同，Go 的 case 默认 break，需要显式 `fallthrough`。
2. **type switch**：`switch v := x.(type)` 可以匹配接口的底层类型。
3. **无条件 switch**：`switch { case ...: }` 等同于 if/else if 链。
4. **case 可以有多个值**：`case 1, 2, 3:` 匹配多个值。

## Rust 特色

1. **match 极其强大**：支持解构、守卫条件、绑定等。
2. **穷尽性检查**：编译器确保所有可能都被覆盖。
3. **模式守卫**：`n if n > 0 =>` 可以在模式后加条件。
4. **解构匹配**：可以匹配并解构元组、结构体、枚举等。

## 运行方式

```bash
# Zig
zig run main.zig

# Go
go run main.go

# Rust
rustc main.rs -o main_rs && ./main_rs
```
