# 23 - 网络编程 (Networking)

网络编程是构建分布式系统和服务的基础。Zig、Go、Rust 三种语言在网络 API 设计上各有特色，但底层都基于操作系统的 socket 接口。

## 核心概念对比

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| TCP 监听 | `std.net.Address.listen()` | `net.Listen("tcp", addr)` | `TcpListener::bind(addr)` |
| TCP 连接 | `std.net.tcpConnectToHost()` | `net.Dial("tcp", addr)` | `TcpStream::connect(addr)` |
| 读写接口 | `reader()`/`writer()` 方法 | `Read`/`Write` 接口 | `Read`/`Write` trait |
| 地址解析 | `std.net.Address.resolveIp()` | `net.ResolveTCPAddr()` | `ToSocketAddrs` trait |
| 非阻塞 I/O | 通过 `std.posix` 手动设置 | goroutine 自动调度 | async + tokio/mio |
| 并发模型 | 每连接一个线程 | 每连接一个 goroutine | async task 或线程 |
| 哲学 | 直接映射系统调用 | 简洁高层抽象 | 零成本抽象 + async |

## 网络编程哲学

### Zig：贴近系统
Zig 的网络 API 是对 POSIX socket 的薄封装。`std.net` 提供了 `Address`、`Server` 和 `Stream` 等核心类型。你需要手动管理连接的生命周期，包括何时关闭连接、如何处理错误等。这种方式虽然需要更多代码，但行为完全透明可控。

### Go：开箱即用
Go 的 `net` 包提供了高层抽象，`net.Listen` + `Accept` 就能快速搭建服务器。每个连接可以用一个 goroutine 处理，几乎无额外开销。Go 的网络 I/O 在底层使用 epoll/kqueue，但对开发者完全透明。

### Rust：async 生态
Rust 标准库提供了同步的 `std::net`，但生产环境通常使用 tokio 等异步运行时。`TcpListener` 和 `TcpStream` 实现了 `Read`/`Write` trait，与整个 I/O 生态无缝集成。

## 运行示例

```bash
# Zig — 先启动服务端，再在另一个终端启动客户端
zig run main.zig

# Go
go run main.go

# Rust
rustc main.rs -o main && ./main
```

## 关键差异

1. **并发处理连接**：Go 用 goroutine 最轻量，Zig 用 OS 线程最直接，Rust 用 async task 最灵活。
2. **API 层级**：Go 的 API 最高层（直接返回 `Conn` 接口），Zig 最底层（暴露 socket fd），Rust 居中。
3. **错误处理**：Zig 用 error union（`!`），Go 用多返回值 `(conn, err)`，Rust 用 `Result<T, E>`。
4. **读写缓冲**：Zig 需要手动提供缓冲区，Go 的 `bufio` 提供缓冲包装，Rust 的 `BufReader`/`BufWriter` 类似。
