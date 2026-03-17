# 第十六章：延迟执行与资源管理（Defer）

资源管理（打开/关闭文件、分配/释放内存、加锁/解锁）是系统编程中最容易出错的环节。三种语言提供了不同的机制来确保资源被正确释放。

## Zig 的方式

Zig 提供了 `defer` 和 `errdefer` 两个关键字：

- `defer`：在当前**作用域**结束时执行，多个 defer 按**逆序**执行
- `errdefer`：仅在函数返回**错误**时执行（Zig 独创！）

**关键特性**：Zig 的 defer 是**作用域级别**的，不是函数级别的。

## Go 的方式

Go 的 `defer` 在**函数返回**时执行：

- 多个 defer 按 LIFO（后进先出）顺序执行
- defer 的参数在声明时求值（不是执行时）
- 常用于 `defer file.Close()`、`defer mu.Unlock()`

**关键特性**：Go 的 defer 是**函数级别**的，不是作用域级别的。这在循环中需要特别注意。

## Rust 的方式

Rust 没有 `defer` 关键字，而是使用 **RAII（资源获取即初始化）**模式：

- `Drop` trait：当变量离开作用域时自动调用
- 析构函数由编译器自动插入
- `std::mem::drop()` 可以提前释放

**关键特性**：资源释放是**自动的**，由编译器保证。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 机制 | `defer` / `errdefer` | `defer` | `Drop` trait (RAII) |
| 执行时机 | 作用域结束 | 函数返回 | 变量离开作用域 |
| 执行顺序 | 逆序 (LIFO) | 逆序 (LIFO) | 声明的逆序 |
| 错误路径清理 | `errdefer`（独创） | `defer`（无条件） | `Drop`（无条件） |
| 参数求值时机 | 声明时 | 声明时 | N/A |
| 循环中使用 | 安全（作用域级） | 危险（函数级） | 安全（作用域级） |
| 条件清理 | `errdefer` | 需手动 flag | 自动 |
| 显式释放 | 手动调用 | 无 | `drop()` |

## 核心区别图解

```
Zig（作用域级 defer）:
{
    const f = openFile();
    defer f.close();      // 离开这个 {} 就执行
    // 使用 f...
}  // <-- f.close() 在此执行

Go（函数级 defer）:
func foo() {
    f := openFile()
    defer f.Close()       // 在 foo() 返回时执行
    // 使用 f...
}  // <-- f.Close() 在此执行

Rust（RAII）:
{
    let f = File::open("x")?;
    // 使用 f...
}  // <-- f 的 Drop 自动调用
```

## 运行方式

```bash
zig run main.zig
go run main.go
rustc main.rs -o main_rs && ./main_rs
```
