# 24 - HTTP 服务器 (HTTP Server)

HTTP 服务器是 Web 开发的核心。三种语言在 HTTP 支持上的差异很大：Go 的标准库就能构建生产级 HTTP 服务，Zig 标准库提供了基础的 HTTP 实现，Rust 则通常依赖第三方库。

## 核心概念对比

| 特性 | Zig | Go | Rust |
|------|-----|-----|------|
| 标准库 HTTP | `std.http.Server`（基础） | `net/http`（生产级） | 无（仅 TCP） |
| 路由 | 手动匹配路径 | `http.ServeMux` | 第三方（axum/actix-web） |
| 中间件 | 手动实现 | `Handler` 接口包装 | Tower middleware / 框架内置 |
| JSON 支持 | `std.json`（基础） | `encoding/json`（完善） | serde_json（事实标准） |
| 并发模型 | 多线程 | goroutine per request | async task per request |
| 生产可用性 | 实验性 | 开箱即用 | 需要 tokio + hyper/axum |
| 静态文件 | 手动实现 | `http.FileServer` | 框架中间件 |

## HTTP 服务器哲学

### Zig：最小化抽象
Zig 的 `std.http.Server` 提供了底层的 HTTP/1.1 解析和响应能力。你需要手动解析请求路径、处理 header、构建响应体。没有内置路由器、中间件或模板引擎。这与 Zig 的整体哲学一致——提供最少但正确的工具，让开发者完全掌控。

### Go：电池全包
Go 的 `net/http` 可能是所有语言中最强大的标准库 HTTP 实现。`http.HandleFunc` 注册路由处理器，`http.ListenAndServe` 启动服务器，几行代码就能构建一个生产级 HTTP 服务。内置的 `ServeMux` 路由器在 Go 1.22 中得到了大幅增强，支持路径参数和方法匹配。

### Rust：async 生态
Rust 标准库没有 HTTP 支持，社区围绕 tokio 异步运行时构建了完善的 Web 生态。axum（基于 tower 和 hyper）是目前最流行的选择，提供类型安全的路由、extractor 模式和强大的中间件系统。本章使用原始 TCP 手动解析 HTTP 来展示底层原理。

## 运行示例

```bash
# Zig
zig run main.zig
# 然后访问 http://127.0.0.1:8080/

# Go
go run main.go
# 然后访问 http://127.0.0.1:8080/

# Rust
rustc main.rs -o main && ./main
# 然后访问 http://127.0.0.1:8080/
```

可以使用 `curl` 测试：
```bash
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/hello?name=Zig
curl http://127.0.0.1:8080/json
```

## 关键差异

1. **开发效率**：Go 的 `net/http` 让你几分钟内就能搭建 HTTP 服务；Zig 和 Rust（不用框架时）需要处理大量底层细节。
2. **性能**：Zig 和 Rust 在吞吐量上通常优于 Go，因为没有 GC 开销和 goroutine 调度开销。
3. **生态成熟度**：Go 的 HTTP 生态最成熟（标准库即可），Rust 次之（tokio 生态完善），Zig 仍在发展中。
4. **JSON 处理**：Go 通过 struct tag 实现序列化，Rust 用 serde derive 宏，Zig 需要手动构建 JSON 字符串或使用 `std.json`。
