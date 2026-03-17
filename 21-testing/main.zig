const std = @import("std");
const testing = std.testing;

// ============================================================
// 被测代码：一个简单的栈 (Stack)
// ============================================================

fn Stack(comptime T: type) type {
    return struct {
        const Self = @This();

        items: std.ArrayListUnmanaged(T) = .{},
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) Self {
            return Self{
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.items.deinit(self.allocator);
        }

        pub fn push(self: *Self, value: T) !void {
            try self.items.append(self.allocator, value);
        }

        pub fn pop(self: *Self) ?T {
            if (self.items.items.len == 0) return null;
            return self.items.pop();
        }

        pub fn peek(self: *const Self) ?T {
            if (self.items.items.len == 0) return null;
            return self.items.items[self.items.items.len - 1];
        }

        pub fn size(self: *const Self) usize {
            return self.items.items.len;
        }
    };
}

// ============================================================
// 被测代码：简单的数学函数
// ============================================================

fn add(a: i32, b: i32) i32 {
    return a + b;
}

fn factorial(n: u32) u64 {
    if (n == 0) return 1;
    var result: u64 = 1;
    for (1..n + 1) |i| {
        result *= i;
    }
    return result;
}

fn fibonacci(n: u32) u64 {
    if (n <= 1) return n;
    var a: u64 = 0;
    var b: u64 = 1;
    for (2..n + 1) |_| {
        const temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

// ============================================================
// 测试 1: 基本断言
// ============================================================

test "add 基本测试" {
    try testing.expectEqual(@as(i32, 3), add(1, 2));
    try testing.expectEqual(@as(i32, 0), add(-1, 1));
    try testing.expectEqual(@as(i32, -2), add(-1, -1));
}

// ============================================================
// 测试 2: 表驱动测试（类似 Go 的风格）
// ============================================================

test "factorial 表驱动测试" {
    const TestCase = struct {
        input: u32,
        expected: u64,
    };

    const cases = [_]TestCase{
        .{ .input = 0, .expected = 1 },
        .{ .input = 1, .expected = 1 },
        .{ .input = 5, .expected = 120 },
        .{ .input = 10, .expected = 3628800 },
    };

    for (cases) |tc| {
        const result = factorial(tc.input);
        try testing.expectEqual(tc.expected, result);
    }
}

test "fibonacci 表驱动测试" {
    const cases = .{
        .{ 0, 0 },
        .{ 1, 1 },
        .{ 2, 1 },
        .{ 10, 55 },
        .{ 20, 6765 },
    };

    inline for (cases) |tc| {
        try testing.expectEqual(@as(u64, tc[1]), fibonacci(tc[0]));
    }
}

// ============================================================
// 测试 3: 使用 testing.allocator 检测内存泄漏！
// ============================================================

test "Stack 无内存泄漏" {
    // testing.allocator 会在测试结束时检查是否有未释放的内存
    var stack = Stack(i32).init(testing.allocator);
    defer stack.deinit(); // 如果注释掉这行，测试会失败并报告泄漏！

    try stack.push(1);
    try stack.push(2);
    try stack.push(3);

    try testing.expectEqual(@as(usize, 3), stack.size());
    try testing.expectEqual(@as(?i32, 3), stack.pop());
    try testing.expectEqual(@as(?i32, 2), stack.pop());
}

test "Stack push 和 pop" {
    var stack = Stack(i32).init(testing.allocator);
    defer stack.deinit();

    // 空栈 pop 返回 null
    try testing.expectEqual(@as(?i32, null), stack.pop());
    try testing.expectEqual(@as(?i32, null), stack.peek());

    try stack.push(42);
    try testing.expectEqual(@as(?i32, 42), stack.peek());
    try testing.expectEqual(@as(usize, 1), stack.size());

    try testing.expectEqual(@as(?i32, 42), stack.pop());
    try testing.expectEqual(@as(usize, 0), stack.size());
}

// ============================================================
// 测试 4: 字符串比较
// ============================================================

test "字符串操作" {
    const hello = "hello";
    const world = "world";

    // 字符串相等比较
    try testing.expectEqualStrings("hello", hello);

    // 字符串切片
    try testing.expectEqualStrings("hel", hello[0..3]);

    // 确保不相等
    try testing.expect(!std.mem.eql(u8, hello, world));
}

// ============================================================
// 测试 5: 错误测试
// ============================================================

const MathError = error{DivisionByZero};

fn safeDivide(a: i32, b: i32) MathError!i32 {
    if (b == 0) return MathError.DivisionByZero;
    return @divTrunc(a, b);
}

test "safeDivide 正常情况" {
    const result = try safeDivide(10, 2);
    try testing.expectEqual(@as(i32, 5), result);
}

test "safeDivide 除以零" {
    // expectError 检查是否返回了预期的错误
    const result = safeDivide(10, 0);
    try testing.expectError(MathError.DivisionByZero, result);
}

// ============================================================
// main 函数（正常构建时使用）
// ============================================================

pub fn main() void {
    std.debug.print("这是正常的 main 函数。\n", .{});
    std.debug.print("运行 `zig test main.zig` 来执行测试。\n", .{});
}
