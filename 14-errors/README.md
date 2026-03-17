# 第十四章：错误处理（Error Handling）

错误处理是每种语言设计哲学的集中体现。三种语言都拒绝了异常机制，但各自选择了不同的替代方案。

## Zig 的方式

Zig 使用**错误联合类型** `!T`，表示一个值要么是 `T`，要么是一个错误。

- `try`：如果是错误就立即返回（类似 Rust 的 `?`）
- `catch`：提供默认值或处理错误（类似可选类型的 `orelse`）
- `errdefer`：仅在函数返回错误时执行清理操作（Zig 独创）
- 错误集（error set）：编译期已知的错误类型集合

Zig 的错误处理强调**零开销**和**显式性**。

## Go 的方式

Go 使用 `(T, error)` 多返回值模式，这是 Go 最具标志性的设计之一：

- `errors.New` / `fmt.Errorf`：创建错误
- `errors.Is` / `errors.As`：错误比较和类型断言
- `%w` 动词：错误包装（wrapping）

简单直接，但也导致了大量 `if err != nil` 的样板代码。

## Rust 的方式

Rust 使用 `Result<T, E>` 枚举：

- `Ok(T)` 表示成功，`Err(E)` 表示失败
- `?` 运算符：自动传播错误
- `From` trait：自动错误类型转换
- `anyhow` / `thiserror`：社区标准错误处理库

Rust 的错误处理最为严格，编译器确保每个错误都被处理。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 错误类型 | `error` (错误集) | `error` 接口 | 任意实现 `Error` trait |
| 成功/失败联合 | `!T` | `(T, error)` | `Result<T, E>` |
| 错误传播 | `try` | `if err != nil { return err }` | `?` |
| 提供默认值 | `catch` | 手动 `if` | `unwrap_or` |
| 错误清理 | `errdefer` | `defer`（无条件） | `Drop`（RAII） |
| 错误包装 | 不支持 | `fmt.Errorf("%w")` | `From` trait |
| 编译期检查 | 是（必须处理） | 否（可忽略） | 是（必须处理） |
| 错误集合 | 编译期错误集 | 无 | 泛型 `E` |

## 运行方式

```bash
zig run main.zig
go run main.go
rustc main.rs -o main_rs && ./main_rs
```
