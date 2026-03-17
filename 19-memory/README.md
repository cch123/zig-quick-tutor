# 第十九章：内存管理（Memory Management）

内存管理是三种语言最根本的设计差异。这一选择深刻影响了语言的性能特征、安全性和编程模型。

## Zig 的方式

Zig 的核心理念是**没有隐藏的分配**——每一次堆内存分配都是显式的：

- **Allocator 作为参数**：需要分配内存的函数接收 `Allocator` 参数，而不是使用全局分配器
- **多种分配器**：
  - `page_allocator`：直接向 OS 申请页面，最简单
  - `GeneralPurposeAllocator`：通用分配器，带越界/释放检测
  - `ArenaAllocator`：批量分配，一次性释放，适合生命周期统一的场景
  - `FixedBufferAllocator`：在栈上预分配的缓冲区中分配，零系统调用
- `@memcpy` / `@memset`：底层内存操作内建函数
- `defer` 配合 `.free()` / `.deinit()`：手动但安全的释放模式

## Go 的方式

Go 使用**垃圾回收（GC）**，程序员几乎不需要关心内存管理：

- `new(T)`：分配一个 `T`，返回指针
- `make(T, ...)`：分配 slice、map、channel
- 逃逸分析：编译器决定变量分配在栈还是堆
- GC：并发三色标记清除算法
- 代价：GC 暂停、内存开销、延迟不确定

## Rust 的方式

Rust 使用**所有权 + 借用检查器**在编译期管理内存：

- 所有权规则：每个值有且只有一个所有者，所有者离开作用域时值被释放
- `Box<T>`：堆分配的智能指针
- `Rc<T>` / `Arc<T>`：引用计数（单线程/多线程）
- 借用：`&T`（不可变）和 `&mut T`（可变），编译期检查
- 无 GC，无运行时开销

## 对比表

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 内存管理模型 | 手动（显式分配器） | 自动（GC） | 编译期（所有权系统） |
| 堆分配 | `allocator.alloc()` | `new()` / `make()` | `Box::new()` |
| 释放 | `allocator.free()` + `defer` | GC 自动回收 | 离开作用域自动 `Drop` |
| 隐藏分配 | 无（核心设计原则） | 到处都是 | 无（但标准库封装了） |
| 分配器可选 | 是（多种分配器） | 否（固定 runtime） | 有限（全局分配器可替换） |
| 内存安全 | 运行时检测（debug 模式） | GC 保证 | 编译期保证 |
| GC 暂停 | 无 | 有（通常 < 1ms） | 无 |
| 引用计数 | 无内建支持 | 无需（有 GC） | `Rc` / `Arc` |
| 栈分配优化 | 手动控制 | 逃逸分析自动决定 | 默认栈分配 |
| 底层内存操作 | `@memcpy` / `@memset` | `copy()` / 无直接 memset | `ptr::copy` / `slice::fill` |

## 运行方式

```bash
zig run main.zig
go run main.go
rustc main.rs -o main_rs && ./main_rs
```
