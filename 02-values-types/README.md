# 02 - Values & Types (值与类型)

## 概述

类型系统是语言的骨架。Zig、Go 和 Rust 都是静态类型语言，但它们在类型命名、推断规则和编译期计算上有显著差异。

## 编译与运行

```bash
zig run main.zig
go run main.go
rustc main.rs -o main && ./main
```

## 整数类型

三种语言都提供了固定宽度的有符号/无符号整数类型：

| 宽度 | Zig | Go | Rust |
|------|-----|-----|------|
| 8-bit | `i8` / `u8` | `int8` / `uint8` (`byte`) | `i8` / `u8` |
| 16-bit | `i16` / `u16` | `int16` / `uint16` | `i16` / `u16` |
| 32-bit | `i32` / `u32` | `int32` / `uint32` (`rune`) | `i32` / `u32` |
| 64-bit | `i64` / `u64` | `int64` / `uint64` | `i64` / `u64` |
| 128-bit | `i128` / `u128` | 无 | `i128` / `u128` |
| 平台相关 | `usize` / `isize` | `int` / `uint` | `usize` / `isize` |

> Zig 的独特之处：支持任意宽度整数，例如 `u3`、`i12`、`u48`，这在位操作和嵌入式编程中非常有用。

## 浮点类型

| Zig | Go | Rust |
|-----|-----|------|
| `f16` | 无 | 无（有 `f16` nightly） |
| `f32` | `float32` | `f32` |
| `f64` | `float64` | `f64` |
| `f128` | 无 | 无 |

## 布尔类型

三种语言都使用 `true` / `false`，类型名为：

| Zig | Go | Rust |
|-----|-----|------|
| `bool` | `bool` | `bool` |

## 字符类型

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 字符类型 | `u8` | `rune` (`int32`) | `char` (Unicode scalar, 4 bytes) |
| 字符字面量 | `'A'` | `'A'` | `'A'` |
| 本质 | 就是一个整数 | 就是 `int32` 的别名 | 独立类型，保证是有效 Unicode |

## 编译期常量与类型推断

这是三种语言最有趣的差异之一：

### Zig: `comptime_int`

```zig
const x = 42;  // 类型是 comptime_int，不是 i32
const y: i32 = x;  // 在使用时才确定具体类型
```

`comptime_int` 是编译期整数，**没有大小限制**，可以表示任意大的数。只有在赋值给运行时变量时才需要放入具体类型。

### Go: untyped constants

```go
const x = 42  // untyped int constant
var y int32 = x  // 在使用时确定类型
```

Go 的 untyped constant 和 Zig 的 comptime_int 概念非常相似！它们都是"在编译期存在，使用时才绑定到具体类型"的值。

### Rust: 整数推断

```rust
let x = 42;  // 默认推断为 i32
let y: u64 = 42;  // 显式指定
let z = 42u64;  // 后缀指定
```

Rust 没有"无限精度编译期整数"的概念。未指定类型的整数字面量默认推断为 `i32`。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 整数字面量默认类型 | `comptime_int` | untyped constant | `i32` |
| 编译期无限精度 | 是 | 是 (untyped) | 否 |
| 任意宽度整数 | 是 (`u3`, `i12` 等) | 否 | 否 |
| 类型推断关键字 | `const x = ...` / `var x = ...` | `x := ...` | `let x = ...` |
| 显式类型标注 | `const x: i32 = ...` | `var x int32 = ...` | `let x: i32 = ...` |
| 数字分隔符 | `1_000_000` | `1_000_000` (Go 1.13+) | `1_000_000` |

## 要点

1. **Zig 的 `comptime_int` 是杀手锏特性**：编译期计算不受类型宽度限制，只在需要运行时值时才检查是否溢出。
2. 如果你熟悉 Go 的 untyped constant，`comptime_int` 对你来说会很自然。它们解决的是同一个问题。
3. Zig 支持任意宽度整数（如 `u3`），这在嵌入式和网络协议解析中非常实用。
