const std = @import("std");
const http = std.http;
const net = std.net;
const print = std.debug.print;

// ============================================================
// 简单的 HTTP 服务器
// 支持路由：/, /hello, /json
// ============================================================

fn handleRequest(request: *http.Server.Request) !void {
    const target = request.head.target;
    const method = request.head.method;

    print("{s} {s}\n", .{ @tagName(method), target });

    // 路由匹配
    if (std.mem.eql(u8, target, "/")) {
        try serveIndex(request);
    } else if (std.mem.startsWith(u8, target, "/hello")) {
        try serveHello(request, target);
    } else if (std.mem.eql(u8, target, "/json")) {
        try serveJson(request);
    } else {
        try serveNotFound(request);
    }
}

// === 路由处理函数 ===

fn serveIndex(request: *http.Server.Request) !void {
    const body =
        \\<!DOCTYPE html>
        \\<html>
        \\<head><title>Zig HTTP Server</title></head>
        \\<body>
        \\  <h1>Welcome to Zig HTTP Server!</h1>
        \\  <ul>
        \\    <li><a href="/hello?name=World">/hello?name=World</a></li>
        \\    <li><a href="/json">/json</a></li>
        \\  </ul>
        \\</body>
        \\</html>
    ;
    try request.respond(body, .{
        .extra_headers = &.{
            .{ .name = "content-type", .value = "text/html; charset=utf-8" },
        },
    });
}

fn serveHello(request: *http.Server.Request, target: []const u8) !void {
    // 解析 query string 中的 name 参数
    var name: []const u8 = "World";

    if (std.mem.indexOf(u8, target, "?")) |query_start| {
        const query = target[query_start + 1 ..];
        var iter = std.mem.splitScalar(u8, query, '&');
        while (iter.next()) |param| {
            if (std.mem.startsWith(u8, param, "name=")) {
                name = param[5..];
            }
        }
    }

    // 构建响应
    var buf: [256]u8 = undefined;
    const body = std.fmt.bufPrint(&buf, "Hello, {s}!\n", .{name}) catch "Hello!\n";

    try request.respond(body, .{
        .extra_headers = &.{
            .{ .name = "content-type", .value = "text/plain; charset=utf-8" },
        },
    });
}

fn serveJson(request: *http.Server.Request) !void {
    const body =
        \\{"message": "Hello from Zig!", "version": "0.15", "features": ["fast", "safe", "simple"]}
    ;
    try request.respond(body, .{
        .extra_headers = &.{
            .{ .name = "content-type", .value = "application/json" },
        },
    });
}

fn serveNotFound(request: *http.Server.Request) !void {
    const body = "404 Not Found\n";
    try request.respond(body, .{
        .status = .not_found,
        .extra_headers = &.{
            .{ .name = "content-type", .value = "text/plain" },
        },
    });
}

// ============================================================

pub fn main() !void {
    const address = net.Address.parseIp("127.0.0.1", 8080) catch unreachable;

    var tcp_server = try address.listen(.{
        .reuse_address = true,
    });
    defer tcp_server.deinit();

    print("Zig HTTP 服务器运行在 http://127.0.0.1:8080/\n", .{});
    print("按 Ctrl+C 停止\n\n", .{});

    while (true) {
        // 接受连接
        const connection = tcp_server.accept() catch |err| {
            print("接受连接失败: {}\n", .{err});
            continue;
        };

        // 从 TCP 连接创建 Reader 和 Writer
        var read_buf: [8192]u8 = undefined;
        var write_buf: [8192]u8 = undefined;
        var stream_reader = connection.stream.reader(&read_buf);
        var stream_writer = connection.stream.writer(&write_buf);

        // 创建 HTTP 服务器
        var http_server = http.Server.init(stream_reader.interface(), &stream_writer.interface);

        // 读取请求头
        var request = http_server.receiveHead() catch |err| {
            print("解析请求失败: {}\n", .{err});
            continue;
        };

        handleRequest(&request) catch |err| {
            print("处理请求失败: {}\n", .{err});
        };
    }
}
