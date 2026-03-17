const std = @import("std");
const print = std.debug.print;

// 1. comptime 参数实现泛型函数 —— 类型作为编译期参数
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

// 2. anytype —— 编译器自动推断类型
fn double(x: anytype) @TypeOf(x) {
    return x * 2;
}

// 3. 泛型容器：编译期生成特定类型的栈
fn Stack(comptime T: type) type {
    return struct {
        const Self = @This();
        items: [64]T = undefined,
        len: usize = 0,

        fn push(self: *Self, value: T) void {
            if (self.len >= 64) return;
            self.items[self.len] = value;
            self.len += 1;
        }

        fn pop(self: *Self) ?T {
            if (self.len == 0) return null;
            self.len -= 1;
            return self.items[self.len];
        }

        fn peek(self: *const Self) ?T {
            if (self.len == 0) return null;
            return self.items[self.len - 1];
        }
    };
}

// 4. comptime 函数求值 —— 编译期计算斐波那契数
fn fibonacci(comptime n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// 5. 编译期反射 —— 使用 @typeInfo 检查类型信息
fn isSignedInt(comptime T: type) bool {
    return switch (@typeInfo(T)) {
        .int => |info| info.signedness == .signed,
        else => false,
    };
}

// 6. comptime 块 —— 编译期生成查找表
const squares = blk: {
    var table: [10]u32 = undefined;
    for (0..10) |i| {
        table[i] = i * i;
    }
    break :blk table;
};

pub fn main() void {
    // 1. 泛型 max
    print("max(i32): {}\n", .{max(i32, 10, 20)});
    print("max(f64): {d:.1}\n", .{max(f64, 3.14, 2.71)});

    // 2. anytype
    print("double(i32): {}\n", .{double(@as(i32, 21))});
    print("double(f64): {d:.1}\n", .{double(@as(f64, 1.5))});

    // 3. 泛型栈
    var int_stack = Stack(i32){};
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    print("peek: {?}\n", .{int_stack.peek()});
    print("pop: {?}\n", .{int_stack.pop()});
    print("pop: {?}\n", .{int_stack.pop()});

    var str_stack = Stack([]const u8){};
    str_stack.push("hello");
    str_stack.push("world");
    print("string pop: {?s}\n", .{str_stack.pop()});

    // 4. 编译期斐波那契（结果在编译时已经计算好）
    const fib10 = comptime fibonacci(10);
    print("fib(10) = {} (computed at compile time)\n", .{fib10});

    // 5. 编译期反射
    print("i32 is signed: {}\n", .{isSignedInt(i32)});
    print("u32 is signed: {}\n", .{isSignedInt(u32)});
    print("f64 is signed: {}\n", .{isSignedInt(f64)});

    // 6. 编译期查找表
    print("squares[7] = {}\n", .{squares[7]});
}
