# 04 - Functions (函数)

## 概述

函数是程序的基本构建块。三种语言在函数语法上各有特色，尤其在多返回值、错误处理和泛型方面差异显著。

## 编译与运行

```bash
zig run main.zig
go run main.go
rustc main.rs -o main && ./main
```

## 基本函数定义

### Zig

```zig
fn add(a: i32, b: i32) i32 {
    return a + b;
}
```

- 参数类型写在参数名后面（和 Go 类似）
- 返回类型在参数列表后面，没有箭头
- 必须用 `return` 显式返回

### Go

```go
func add(a, b int32) int32 {
    return a + b
}
```

- 同类型参数可以合并声明 (`a, b int32`)

### Rust

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 最后一个表达式自动作为返回值
}
```

- 返回类型用 `->` 标注
- 最后一个表达式（无分号）自动作为返回值

## 多返回值

| 语言 | 机制 | 语法 |
|------|------|------|
| Zig | 匿名结构体 | `fn f() struct { i32, i32 }` |
| Go | 原生多返回值 | `func f() (int32, int32)` |
| Rust | 元组 | `fn f() -> (i32, i32)` |

### Zig

```zig
fn swap(a: i32, b: i32) struct { i32, i32 } {
    return .{ b, a };
}

const result = swap(10, 20);
// 通过索引访问: result[0], result[1]
```

Zig 使用匿名结构体模拟多返回值。通过索引或解构访问各字段。

### Go

```go
x, y := swap(10, 20)
```

Go 的多返回值是语言原生支持的，语法最自然。

### Rust

```rust
let (x, y) = swap(10, 20);
```

Rust 使用元组，通过模式匹配解构。

## 错误处理

这是三种语言差异最大的地方。

| 语言 | 错误类型 | 语法 | 传播语法 |
|------|----------|------|----------|
| Zig | `error` union: `!T` | `MathError!i32` | `try` |
| Go | `error` 接口 | `(T, error)` | `if err != nil { return err }` |
| Rust | `Result<T, E>` | `Result<i32, MathError>` | `?` |

### Zig 的错误联合体

```zig
const MathError = error{ DivisionByZero, Overflow };

fn safeDivide(a: i32, b: i32) MathError!i32 {
    if (b == 0) return MathError.DivisionByZero;
    return @divTrunc(a, b);
}

// 使用 if-else 解包
if (safeDivide(10, 3)) |val| {
    // 成功路径
} else |err| {
    // 错误路径
}

// 使用 catch 提供默认值
const val = safeDivide(10, 0) catch 0;

// 使用 try 传播错误（在错误返回函数中）
// const val = try safeDivide(10, 3);
```

`!T` 是 Zig 的错误联合类型。`MathError!i32` 表示"要么是 `i32`，要么是 `MathError`"。和 Go 的 `(T, error)` 相比：
- 不需要检查 `err != nil`，编译器强制你处理
- `try` 关键字等同于 Go 里的 `if err != nil { return err }`，但只有一个词

### Go 的错误处理

```go
val, err := safeDivide(10, 3)
if err != nil {
    return err
}
```

你最熟悉的模式。简单直接，但冗长。

### Rust 的 Result

```rust
fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 { return Err(MathError::DivisionByZero); }
    Ok(a / b)
}

// match 解包
match safe_divide(10, 3) {
    Ok(val) => println!("{}", val),
    Err(e) => println!("{}", e),
}

// ? 操作符传播错误
// let val = safe_divide(10, 3)?;
```

Rust 的 `?` 操作符和 Zig 的 `try` 功能相同。

## 函数指针与一等函数

三种语言都支持函数作为一等公民：

| 语言 | 函数指针类型 | 传递方式 |
|------|------------|----------|
| Zig | `*const fn (i32, i32) i32` | `&multiply` |
| Go | `func(int32, int32) int32` | `multiply` |
| Rust | `fn(i32, i32) -> i32` | `multiply` |

Go 和 Rust 传递函数时不需要取地址，Zig 需要用 `&` 取函数指针。

## Comptime 参数（Zig 独有）

```zig
fn repeat(comptime n: usize, value: u8) [n]u8 {
    return [_]u8{value} ** n;
}

fn maxOf(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}
```

`comptime` 参数必须在编译期已知。这让 Zig 实现了：

1. **编译期泛型**：`comptime T: type` 让函数接受任意类型，类似 Go 泛型和 Rust 泛型，但实现机制不同
2. **编译期值参数**：`comptime n: usize` 让返回类型依赖于参数值（`[n]u8`），这在 Go 和 Rust 中都做不到
3. **零成本抽象**：所有 comptime 参数在编译期展开，运行时没有任何开销

### 与 Go/Rust 泛型对比

| 特性 | Zig `comptime` | Go 泛型 | Rust 泛型 |
|------|---------------|---------|----------|
| 语法 | `comptime T: type` | `[T any]` | `<T: Trait>` |
| 约束 | duck typing（编译期检查） | 接口约束 | trait 约束 |
| 单态化 | 是 | 部分（GC shape stenciling） | 是 |
| 值参数 | 是 (`comptime n: usize`) | 否 | 有限 (const generics) |
| 编译期执行 | 任意代码 | 否 | 有限 (`const fn`) |

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 函数关键字 | `fn` | `func` | `fn` |
| 返回类型位置 | 参数后直接写 | 参数后直接写 | `-> T` |
| 隐式返回 | 否 | 否 | 是（最后表达式） |
| 多返回值 | 匿名结构体 | 原生支持 | 元组 |
| 错误处理 | `!T` + `try`/`catch` | `(T, error)` + `if err` | `Result<T,E>` + `?` |
| 泛型 | `comptime T: type` | `[T Constraint]` | `<T: Trait>` |
| 函数指针 | `*const fn(T) U` | `func(T) U` | `fn(T) -> U` |

## 要点

1. **Zig 的 `try` = Go 的 `if err != nil { return err }` = Rust 的 `?`**。三行 Go 代码在 Zig 和 Rust 中都可以用一个关键字/操作符替代。
2. **Zig 的错误联合 `!T` 强制错误处理**：不像 Go 可以用 `_` 忽略 error 返回值，Zig 的编译器不允许你忽略错误。
3. **`comptime` 是 Zig 最强大的特性之一**：它不仅能做泛型，还能做编译期代码生成，这让 Zig 在保持简单语法的同时拥有了极强的表达力。
4. 作为 Go 开发者，Zig 的函数语法会感觉非常熟悉（参数类型在名字后面，返回类型在参数后面），只是关键字从 `func` 变成了 `fn`。
