# 20 - 并发编程 (Concurrency)

并发是现代编程中不可避免的话题。Zig、Go、Rust 三种语言在并发模型上有着截然不同的哲学。

## 核心概念对比

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 基本并发单元 | OS 线程 (`std.Thread`) | goroutine（用户态协程） | OS 线程 (`std::thread`) |
| 异步模型 | 无内置 async（已移除） | goroutine + channel | async/await + 运行时（tokio 等） |
| 通信方式 | 共享内存 + Mutex | channel（CSP 模型） | channel (`mpsc`) / 共享内存 |
| 互斥锁 | `std.Thread.Mutex` | `sync.Mutex` | `std::sync::Mutex<T>` |
| 条件变量 | `std.Thread.Condition` | `sync.Cond` | `std::sync::Condvar` |
| 线程池 | `std.Thread.Pool` | runtime 自动管理 | rayon / tokio runtime |
| 线程安全检查 | 无编译期检查 | race detector（运行时） | 编译期 `Send`/`Sync` trait |
| 哲学 | 简单直接，手动控制 | "不要通过共享内存通信" | "无畏并发"，编译期保证 |

## 并发模型哲学

### Zig：简单直接
Zig 在 0.14 版本中移除了 async/await，选择回归最简单的 OS 线程模型。Zig 的哲学是"没有隐藏的控制流"——你创建的每个线程都是真实的 OS 线程，行为完全可预测。`std.Thread.Pool` 提供了线程池来避免频繁创建销毁线程的开销。

### Go：goroutine + channel
Go 的并发模型是其最大的亮点。goroutine 是由 Go runtime 调度的轻量级协程，创建成本极低（几 KB 栈空间）。channel 是一等公民，Go 鼓励通过 channel 传递数据而非共享内存。`select` 语句可以同时等待多个 channel 操作。

### Rust：编译期安全保证
Rust 通过 `Send` 和 `Sync` trait 在编译期保证线程安全——如果你的代码能通过编译，就不会有数据竞争。标准库提供 OS 线程，异步编程则依赖第三方运行时（如 tokio）。`Arc<Mutex<T>>` 是共享可变状态的标准模式。

## 运行示例

```bash
# Zig
zig run main.zig

# Go
go run main.go

# Rust (需要在 Cargo 项目中添加依赖，此处用标准库演示)
rustc main.rs -o main && ./main
```

## 关键差异

1. **goroutine vs OS 线程**：Go 可以轻松创建数十万个 goroutine，而 Zig/Rust 创建 OS 线程的成本较高（每个线程默认 8MB 栈空间）。
2. **安全性**：Rust 在编译期就能阻止数据竞争，Go 提供运行时 race detector，Zig 不提供任何自动检查。
3. **async/await**：Zig 选择移除，Go 不需要（goroutine 天然异步），Rust 全面拥抱但需要第三方运行时。
4. **Channel**：Go 的 channel 是语言内置的，Rust 的 `mpsc` 在标准库中，Zig 没有内置 channel（需要自己用 Mutex + Condition 实现）。
