# 第十八章：接口与 Trait（Interfaces & Traits）

多态是面向对象编程的核心，但三种语言对此有着完全不同的理念。

## Zig 的方式

Zig **没有接口、没有 trait、没有继承**。它通过以下方式实现多态：

- **comptime duck typing**：使用 `anytype` 参数，编译器在调用点检查类型是否满足要求
- **手动虚表（fat pointer）**：通过函数指针 + 类型擦除指针手动构造接口，这是 `std.mem.Allocator` 等标准库类型的实现方式
- **tagged union**：用标签联合实现有限多态
- **函数指针字段**：结构体中存储函数指针

Zig 的设计选择是：**显式优于隐式**，宁可多写几行代码，也不要隐藏运行时开销。

## Go 的方式

Go 的接口是其最优雅的设计之一：

- **隐式满足**：不需要显式声明实现了某个接口
- **接口组合**：小接口组合成大接口（如 `io.ReadWriter`）
- `any`（`interface{}`）：空接口，可以持有任何值
- **类型断言 / type switch**：运行时检查具体类型

Go 的接口是动态分派（虚表调用），有运行时开销。

## Rust 的方式

Rust 使用 trait 系统：

- **显式实现**：必须用 `impl Trait for Type` 声明
- **静态分派**：`impl Trait` 参数，编译期单态化，零开销
- **动态分派**：`dyn Trait`，通过 trait object 实现，有虚表开销
- **trait 组合**：`T: Read + Write`
- **默认方法**：trait 中可以提供默认实现

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 多态机制 | comptime / 手动虚表 / tagged union | 接口（隐式满足） | trait（显式实现） |
| 静态分派 | anytype（编译期） | 无 | `impl Trait` / 泛型 |
| 动态分派 | 手动函数指针 | 接口调用（自动） | `dyn Trait` |
| 接口声明 | 无语法支持 | `type I interface { ... }` | `trait T { ... }` |
| 隐式/显式 | 编译期鸭子类型 | 隐式满足 | 显式 `impl` |
| 默认实现 | 无 | 无 | 支持 |
| 虚表开销 | 手动控制，可选 | 始终存在 | `dyn` 时存在 |
| 类型擦除 | `*anyopaque` + 函数指针 | `interface{}` / `any` | `Box<dyn Trait>` |
| 运行时反射 | 无 | `reflect` 包 | 无（有限的 `Any`） |

## 运行方式

```bash
zig run main.zig
go run main.go
rustc main.rs -o main_rs && ./main_rs
```
