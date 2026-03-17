const std = @import("std");
const print = std.debug.print;

// 1. 定义错误集
const FileError = error{
    NotFound,
    PermissionDenied,
    TooLarge,
};

// 2. 函数返回错误联合类型 !T
fn divide(a: f64, b: f64) !f64 {
    if (b == 0) return error.DivisionByZero;
    return a / b;
}

// 3. 使用特定错误集
fn readConfig(path: []const u8) FileError![]const u8 {
    if (std.mem.eql(u8, path, "missing.txt")) {
        return FileError.NotFound;
    }
    if (std.mem.eql(u8, path, "secret.txt")) {
        return FileError.PermissionDenied;
    }
    return "config_data";
}

// 4. try 关键字 - 如果是错误就向上传播
fn loadAndProcess(path: []const u8) ![]const u8 {
    // try = 如果 readConfig 返回错误，立即返回该错误
    const data = try readConfig(path);
    _ = data;
    return "processed_data";
}

// 5. errdefer - 仅在错误路径执行
fn allocateAndProcess(allocator: std.mem.Allocator) ![]u8 {
    const buf = try allocator.alloc(u8, 100);
    // 如果后续代码返回错误，释放已分配的内存
    errdefer allocator.free(buf);

    // 模拟可能失败的操作
    if (buf.len > 50) {
        return error.TooLarge;
    }
    return buf;
}

pub fn main() void {
    // 1. try 的基本使用（main 中不能用 try，所以用 catch）
    const result = divide(10, 3) catch |err| {
        print("Error: {}\n", .{err});
        return;
    };
    print("10 / 3 = {d:.2}\n", .{result});

    // 2. catch 提供默认值
    const safe_result = divide(10, 0) catch 0;
    print("10 / 0 (with default) = {d:.2}\n", .{safe_result});

    // 3. catch 捕获并处理错误
    const config = readConfig("missing.txt") catch |err| blk: {
        print("Config error: {}\n", .{err});
        break :blk "default_config";
    };
    _ = config;

    // 4. switch 处理不同错误
    const config2 = readConfig("secret.txt") catch |err| switch (err) {
        FileError.NotFound => blk: {
            print("File not found, using defaults\n", .{});
            break :blk "default";
        },
        FileError.PermissionDenied => blk: {
            print("Permission denied!\n", .{});
            break :blk "denied";
        },
        FileError.TooLarge => blk: {
            print("File too large!\n", .{});
            break :blk "too_large";
        },
    };
    print("Config2: {s}\n", .{config2});

    // 5. errdefer 演示
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // 这会触发 errdefer，自动释放内存
    const data = allocateAndProcess(allocator) catch |err| {
        print("allocateAndProcess failed: {} (memory was freed by errdefer)\n", .{err});
        return;
    };
    defer allocator.free(data);

    // 6. 成功路径
    const good_config = readConfig("normal.txt") catch unreachable;
    print("Good config: {s}\n", .{good_config});
}
