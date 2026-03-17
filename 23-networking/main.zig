const std = @import("std");
const net = std.net;
const print = std.debug.print;

// ============================================================
// 示例 1: TCP Echo 服务器
// ============================================================

fn handleClient(stream: net.Stream) void {
    defer stream.close();

    const addr = stream.getLocalAddress();
    print("客户端已连接: {}\n", .{addr});

    var buf: [1024]u8 = undefined;

    // 循环读取客户端数据并回显
    while (true) {
        const bytes_read = stream.read(&buf) catch |err| {
            print("读取错误: {}\n", .{err});
            return;
        };

        // 客户端关闭连接
        if (bytes_read == 0) {
            print("客户端断开连接\n", .{});
            return;
        }

        const data = buf[0..bytes_read];
        print("收到: {s}", .{data});

        // 回显数据
        _ = stream.writeAll(data) catch |err| {
            print("写入错误: {}\n", .{err});
            return;
        };
    }
}

fn echoServer() !void {
    print("=== TCP Echo 服务器 ===\n", .{});

    // 解析监听地址
    const address = net.Address.parseIp("127.0.0.1", 8080) catch unreachable;

    // 创建 TCP 服务器
    var server = try address.listen(.{
        .reuse_address = true,
    });
    defer server.deinit();

    print("服务器监听在 127.0.0.1:8080\n", .{});

    // 接受连接（这里只处理 3 个连接作为演示）
    for (0..3) |_| {
        const connection = try server.accept();
        // 为每个连接创建一个线程
        const thread = try std.Thread.spawn(.{}, handleClient, .{connection.stream});
        thread.detach();
    }

    // 等待一下让线程处理完
    std.Thread.sleep(1 * std.time.ns_per_s);
    print("服务器关闭\n\n", .{});
}

// ============================================================
// 示例 2: TCP 客户端
// ============================================================

fn tcpClient() !void {
    print("=== TCP 客户端 ===\n", .{});

    // 连接到服务器
    const stream = try net.tcpConnectToHost(
        std.heap.page_allocator,
        "127.0.0.1",
        8080,
    );
    defer stream.close();

    print("已连接到服务器\n", .{});

    // 发送数据
    const message = "Hello, Zig Networking!\n";
    try stream.writeAll(message);
    print("发送: {s}", .{message});

    // 读取回显
    var buf: [1024]u8 = undefined;
    const bytes_read = try stream.read(&buf);
    print("收到回显: {s}\n", .{buf[0..bytes_read]});
}

// ============================================================
// 示例 3: 简单的请求-响应服务器（模拟协议）
// ============================================================

const Request = struct {
    command: []const u8,
    payload: []const u8,
};

fn parseRequest(data: []const u8) ?Request {
    // 简单协议：COMMAND:PAYLOAD
    const sep = std.mem.indexOf(u8, data, ":") orelse return null;
    return Request{
        .command = data[0..sep],
        .payload = data[sep + 1 ..],
    };
}

fn handleRequest(stream: net.Stream) void {
    defer stream.close();

    var buf: [1024]u8 = undefined;
    const bytes_read = stream.read(&buf) catch return;
    if (bytes_read == 0) return;

    const data = buf[0..bytes_read];

    if (parseRequest(data)) |req| {
        print("命令: {s}, 数据: {s}\n", .{ req.command, req.payload });

        // 根据命令生成响应
        if (std.mem.eql(u8, req.command, "PING")) {
            stream.writeAll("PONG\n") catch return;
        } else if (std.mem.eql(u8, req.command, "ECHO")) {
            stream.writeAll(req.payload) catch return;
        } else {
            stream.writeAll("UNKNOWN COMMAND\n") catch return;
        }
    } else {
        stream.writeAll("INVALID REQUEST\n") catch return;
    }
}

fn requestResponseDemo() !void {
    print("\n=== 请求-响应服务器 ===\n", .{});

    const address = net.Address.parseIp("127.0.0.1", 8081) catch unreachable;
    var server = try address.listen(.{
        .reuse_address = true,
    });
    defer server.deinit();

    print("协议服务器监听在 127.0.0.1:8081\n", .{});

    // 在另一个线程中发送测试请求
    const client_thread = try std.Thread.spawn(.{}, struct {
        fn run() void {
            // 等待服务器启动
            std.Thread.sleep(100 * std.time.ns_per_ms);

            // 发送 PING 请求
            if (net.tcpConnectToHost(std.heap.page_allocator, "127.0.0.1", 8081)) |stream| {
                defer stream.close();
                stream.writeAll("PING:hello") catch return;

                var buf: [1024]u8 = undefined;
                const n = stream.read(&buf) catch return;
                print("客户端收到: {s}\n", .{buf[0..n]});
            } else |_| {}

            // 发送 ECHO 请求
            if (net.tcpConnectToHost(std.heap.page_allocator, "127.0.0.1", 8081)) |stream| {
                defer stream.close();
                stream.writeAll("ECHO:Hello World!\n") catch return;

                var buf: [1024]u8 = undefined;
                const n = stream.read(&buf) catch return;
                print("客户端收到: {s}", .{buf[0..n]});
            } else |_| {}
        }
    }.run, .{});

    // 服务器处理 2 个请求
    for (0..2) |_| {
        const connection = try server.accept();
        handleRequest(connection.stream);
    }

    client_thread.join();
    print("请求-响应演示完成\n", .{});
}

// ============================================================

pub fn main() !void {
    // 注意：echoServer 需要外部客户端连接，这里只运行自包含的演示
    // 如需测试 echo 服务器，取消下面的注释并用 telnet/nc 连接
    // try echoServer();
    // try tcpClient();

    try requestResponseDemo();
}
