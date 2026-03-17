const std = @import("std");
const print = std.debug.print;

// 模拟资源：一个简单的"日志记录器"
const Logger = struct {
    name: []const u8,

    fn init(name: []const u8) Logger {
        print("[{s}] opened\n", .{name});
        return Logger{ .name = name };
    }

    fn write(self: Logger, msg: []const u8) void {
        print("[{s}] {s}\n", .{ self.name, msg });
    }

    fn close(self: Logger) void {
        print("[{s}] closed\n", .{self.name});
    }
};

// 1. 基本 defer 使用
fn basicDefer() void {
    print("--- basicDefer ---\n", .{});
    const logger = Logger.init("basic");
    defer logger.close(); // 函数返回前关闭

    logger.write("doing work...");
    logger.write("more work...");
    // logger.close() 会在此自动执行
}

// 2. 多个 defer 按逆序执行
fn multipleDefers() void {
    print("\n--- multipleDefers ---\n", .{});
    defer print("first defer (executes last)\n", .{});
    defer print("second defer (executes second)\n", .{});
    defer print("third defer (executes first)\n", .{});
    print("main body\n", .{});
}

// 3. 作用域级 defer（与 Go 的关键区别！）
fn scopedDefer() void {
    print("\n--- scopedDefer ---\n", .{});

    var i: usize = 0;
    while (i < 3) : (i += 1) {
        // 这个 defer 在每次循环迭代结束时执行
        // （不是在函数返回时！）
        const logger = Logger.init("loop");
        defer logger.close();

        logger.write("iteration work");
        // logger.close() 在每次迭代结束时执行
    }
    print("After loop\n", .{});
}

// 4. errdefer - 仅在错误路径执行（Zig 独创！）
fn riskyOperation(should_fail: bool) ![]const u8 {
    print("\n--- riskyOperation(fail={}) ---\n", .{should_fail});

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const buf = try allocator.alloc(u8, 32);
    // 仅在返回错误时释放 buf
    errdefer {
        print("errdefer: freeing buffer due to error!\n", .{});
        allocator.free(buf);
    }

    if (should_fail) {
        return error.OperationFailed;
        // errdefer 会执行，释放 buf
    }

    @memcpy(buf[0..5], "hello");
    // 成功路径：errdefer 不执行，调用者负责释放
    return buf[0..5];
}

// 5. defer 在块作用域中
fn blockScopeDemo() void {
    print("\n--- blockScopeDemo ---\n", .{});

    print("before block\n", .{});
    {
        defer print("defer in inner block\n", .{});
        print("inside block\n", .{});
    } // <-- 内部 defer 在此执行
    print("after block\n", .{});
}

pub fn main() void {
    // 1. 基本 defer
    basicDefer();

    // 2. 多个 defer 逆序执行
    multipleDefers();

    // 3. 作用域级 defer（循环安全）
    scopedDefer();

    // 4. errdefer 演示
    // 失败路径：errdefer 执行
    _ = riskyOperation(true) catch |err| {
        print("Caught error: {}\n", .{err});
    };

    // 成功路径：errdefer 不执行
    if (riskyOperation(false)) |result| {
        print("Success: {s}\n", .{result});
        // 注意：实际代码中应该释放 result 的内存
    } else |err| {
        print("Error: {}\n", .{err});
    }

    // 5. 块作用域 defer
    blockScopeDemo();
}
