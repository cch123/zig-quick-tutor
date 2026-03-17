# 第十七章：泛型与编译期计算（Generics / Comptime）

泛型是现代编程语言的核心特性之一。三种语言采用了截然不同的方式来实现类型参数化。

## Zig 的方式

Zig 没有传统意义上的泛型——它使用 **comptime（编译期计算）** 来实现同样的效果，而且更加强大：

- `comptime` 参数：函数参数在编译期求值，类型本身可以作为值传递
- `anytype`：类似于无约束泛型，编译器在调用点推断类型
- `@TypeOf`：编译期获取表达式的类型
- `@typeInfo`：编译期反射，获取类型的详细信息
- comptime 块：在编译期执行任意代码（包括循环、条件判断等）

Zig 的设计哲学是：**泛型只是编译期计算的一个特例**。

## Go 的方式

Go 在 1.18 版本引入了泛型：

- `[T any]`：类型参数声明
- `constraints`：类型约束（如 `comparable`、`constraints.Ordered`）
- `~` 运算符：底层类型约束
- 接口作为约束：用接口定义类型必须满足的方法集

Go 的泛型设计相对保守，强调简单性。

## Rust 的方式

Rust 从一开始就有强大的泛型系统：

- `<T>`：类型参数
- trait bounds：`T: Display + Clone`
- `where` 子句：复杂约束
- 单态化（monomorphization）：编译期生成具体类型的代码

Rust 的泛型是零开销抽象的核心。

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 泛型机制 | comptime 参数 | 类型参数 `[T any]` | 类型参数 `<T>` |
| 类型约束 | 编译期检查（duck typing） | 接口约束 | trait bounds |
| 编译期计算 | comptime 块，极其强大 | 无 | const fn（有限） |
| 类型作为值 | `type` 是一等公民 | 否 | 否 |
| 编译期反射 | `@typeInfo`，完整反射 | 无 | 无（需要宏） |
| 零开销 | 是（编译期展开） | 部分（字典传递 / 单态化混合） | 是（单态化） |
| 编译期函数求值 | 任意函数 | 无 | const fn（受限） |
| 可变参数泛型 | 通过 comptime 切片实现 | 无 | 无（需要宏） |

## 运行方式

```bash
zig run main.zig
go run main.go
rustc main.rs -o main_rs && ./main_rs
```
