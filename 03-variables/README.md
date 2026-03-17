# 03 - Variables (变量)

## 概述

变量声明和可变性是理解一门语言的基础。三种语言在"默认可变还是不可变"的设计哲学上有本质区别。

## 编译与运行

```bash
zig run main.zig
go run main.go
rustc main.rs -o main && ./main
```

## 变量声明

### Zig

```zig
var x: i32 = 10;    // 可变
const y: i32 = 42;  // 不可变（优先使用）
const z = 100;       // 类型推断
```

Zig 编译器会**强制**你使用 `const`，除非你确实需要修改变量。如果一个 `var` 变量从未被修改，编译器会报错。

### Go

```go
var x int32 = 10  // 显式类型
y := 42           // 短声明 + 类型推断
```

Go 没有不可变变量的概念。所有局部变量都是可变的。

### Rust

```rust
let x: i32 = 10;      // 不可变（默认）
let mut y: i32 = 42;  // 可变（需要 mut）
let z = 100;           // 类型推断
```

Rust 和 Zig 类似，默认不可变。但语法相反：Zig 用 `var` 表示可变，Rust 用 `let mut`。

## 常量

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 语法 | `const` (+ `comptime`) | `const` | `const` |
| 必须标注类型 | 否 | 否 | 是 |
| 可用于复杂计算 | 是 (`comptime` 块) | 仅限常量表达式 | 仅限常量表达式 |

### Zig 的 `comptime`

```zig
const val = comptime blk: {
    var result: i32 = 1;
    var i: i32 = 1;
    while (i <= 5) : (i += 1) {
        result *= i;
    }
    break :blk result;
};
```

Zig 的 `comptime` 可以在编译期执行**任意代码**（循环、分支等），这比 Go 和 Rust 的 `const` 强大得多。Go 的 `const` 只支持简单表达式，Rust 的 `const fn` 有诸多限制。

## Shadowing（变量遮蔽）

| 语言 | 允许 Shadowing | 说明 |
|------|---------------|------|
| Zig | **不允许** | 同作用域内不能重复声明同名变量 |
| Go | **允许** | 内层作用域可以遮蔽外层变量 |
| Rust | **允许** | 同作用域也可以 `let` 重新绑定同名变量，甚至可以改变类型 |

### 为什么 Zig 不允许 Shadowing？

Zig 认为 Shadowing 容易导致 bug。如果你在内层作用域意外使用了和外层相同的变量名，可能会产生难以发现的错误。这和 Zig "没有隐藏行为"的设计哲学一致。

### Go 的 Shadowing 陷阱

```go
err := doSomething()
if err != nil {
    result, err := doAnotherThing()  // 这个 err 是新变量！
    // 外层的 err 没有被修改
}
```

这是 Go 中一个经典的 bug 来源。

## 未初始化与零值

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 未初始化 | `var x: i32 = undefined` | 不适用 | 不允许使用未初始化变量 |
| 零值 | 无自动零值 | 所有类型有零值 | 无自动零值 |
| 延迟初始化 | 支持 (`undefined`) | 不需要（有零值） | 支持（编译器做流分析） |

### Zig 的 `undefined`

```zig
var x: i32 = undefined;  // 未初始化，内存内容不确定
x = 42;                   // 使用前必须赋值
```

`undefined` 是 Zig 独特的概念。它告诉编译器"我知道这个变量还没有值，我保证会在使用前赋值"。如果你忘了赋值就使用，在 Debug 模式下会触发安全检查。

### Go 的零值

```go
var i int     // 0
var f float64 // 0.0
var b bool    // false
var s string  // ""
var p *int    // nil
```

Go 的零值系统是最"友好"的 —— 你永远不会遇到未初始化的变量。但代价是你无法区分"有意设为零"和"忘记初始化"。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 可变声明 | `var x: T = val` | `var x T = val` / `x := val` | `let mut x: T = val` |
| 不可变声明 | `const x: T = val` | 无（局部变量总是可变） | `let x: T = val` |
| 默认可变性 | 不可变 (`const`) | 可变 | 不可变 (`let`) |
| Shadowing | 不允许 | 允许 | 允许（且可改类型） |
| 未初始化 | `undefined` | 零值 | 编译错误 |
| 编译期计算 | `comptime` 块 | 常量表达式 | `const fn` (受限) |

## 要点

1. **Zig 和 Rust 都倡导默认不可变**。Go 是个例外 —— 所有局部变量都可变。
2. **Zig 的 `comptime` 远超 Go 的 `const`**：可以执行循环、调用函数、操作类型，是 Zig 的核心特性之一。
3. **`undefined` 不是 `nil`/`null`**：它表示"内存未初始化"，不是"空值"。这是一个底层概念，小心使用。
4. 作为 Go 开发者，记住：Zig 里 `const` 对应你的 `:=`（不可变绑定），`var` 才是你的 `var`（可变变量）。
