const std = @import("std");

// === Tagged union：Zig 最强大的特性之一 ===
const Shape = union(enum) {
    circle: f64,
    rectangle: struct { width: f64, height: f64 },
    triangle: struct { base: f64, height: f64 },

    pub fn area(self: Shape) f64 {
        return switch (self) {
            .circle => |r| std.math.pi * r * r,
            .rectangle => |rect| rect.width * rect.height,
            .triangle => |tri| 0.5 * tri.base * tri.height,
        };
    }

    pub fn name(self: Shape) []const u8 {
        return switch (self) {
            .circle => "Circle",
            .rectangle => "Rectangle",
            .triangle => "Triangle",
        };
    }
};

// === 带自定义 tag 类型的 tagged union ===
const Token = union(enum) {
    number: f64,
    string: []const u8,
    plus,
    minus,
    eof,

    pub fn format(self: Token, writer: anytype) !void {
        switch (self) {
            .number => |n| try writer.print("Number({d})", .{n}),
            .string => |s| try writer.print("String(\"{s}\")", .{s}),
            .plus => try writer.print("Plus", .{}),
            .minus => try writer.print("Minus", .{}),
            .eof => try writer.print("EOF", .{}),
        }
    }
};

// === extern union：C 互操作 ===
const CValue = extern union {
    int_val: i32,
    float_val: f32,
    // 注意：extern union 没有 tag，访问错误的字段是未定义行为
};

// === 普通 union（裸联合体） ===
const Number = union {
    int: i64,
    float: f64,
};

// === 可选类型 ?T 本质上是 tagged union ===
fn findIndex(haystack: []const u8, needle: u8) ?usize {
    for (haystack, 0..) |byte, i| {
        if (byte == needle) return i;
    }
    return null;
}

// === 错误联合体 !T 也类似 tagged union ===
const ParseError = error{
    InvalidCharacter,
    Overflow,
};

fn parseDigit(c: u8) ParseError!u8 {
    if (c >= '0' and c <= '9') {
        return c - '0';
    }
    return ParseError.InvalidCharacter;
}

pub fn main() !void {
    var buf: [4096]u8 = undefined;
    const file = std.fs.File{ .handle = std.posix.STDOUT_FILENO };
    var w = file.writer(&buf);
    defer w.interface.flush() catch {};

    // === Tagged union 基本使用 ===
    const shapes = [_]Shape{
        .{ .circle = 5.0 },
        .{ .rectangle = .{ .width = 4.0, .height = 6.0 } },
        .{ .triangle = .{ .base = 3.0, .height = 8.0 } },
    };

    try w.interface.print("=== 形状面积 ===\n", .{});
    for (shapes) |shape| {
        try w.interface.print("  {s}: {d:.2}\n", .{ shape.name(), shape.area() });
    }

    // === switch 捕获载荷 ===
    const shape = shapes[0];
    switch (shape) {
        .circle => |radius| {
            try w.interface.print("\n圆的半径: {d}\n", .{radius});
        },
        .rectangle => |rect| {
            try w.interface.print("\n矩形: {}x{}\n", .{ rect.width, rect.height });
        },
        .triangle => |tri| {
            try w.interface.print("\n三角形: base={}, h={}\n", .{ tri.base, tri.height });
        },
    }

    // === Token 示例 ===
    const tokens = [_]Token{
        .{ .number = 42 },
        .plus,
        .{ .number = 8 },
        .eof,
    };

    try w.interface.print("\n=== 词法标记 ===\n", .{});
    for (tokens) |token| {
        try w.interface.print("  {f}\n", .{token});
    }

    // === 可选类型 ===
    try w.interface.print("\n=== 可选类型 ?T ===\n", .{});
    const data = "Hello, World!";
    if (findIndex(data, 'W')) |index| {
        try w.interface.print("找到 'W' 在位置: {}\n", .{index});
    } else {
        try w.interface.print("未找到\n", .{});
    }

    if (findIndex(data, 'Z')) |index| {
        try w.interface.print("找到 'Z' 在位置: {}\n", .{index});
    } else {
        try w.interface.print("未找到 'Z'\n", .{});
    }

    // orelse 提供默认值
    const idx = findIndex(data, 'Z') orelse 0;
    try w.interface.print("orelse 默认值: {}\n", .{idx});

    // === 错误联合体 ===
    try w.interface.print("\n=== 错误联合体 !T ===\n", .{});
    if (parseDigit('5')) |digit| {
        try w.interface.print("解析 '5': {}\n", .{digit});
    } else |err| {
        try w.interface.print("错误: {}\n", .{err});
    }

    if (parseDigit('x')) |digit| {
        try w.interface.print("解析 'x': {}\n", .{digit});
    } else |err| {
        try w.interface.print("解析 'x' 错误: {}\n", .{err});
    }

    // try 语法糖
    const digit = try parseDigit('7');
    try w.interface.print("try 解析 '7': {}\n", .{digit});

    // === 内存布局 ===
    try w.interface.print("\n=== 内存大小 ===\n", .{});
    try w.interface.print("Shape 大小: {} 字节\n", .{@sizeOf(Shape)});
    try w.interface.print("Token 大小: {} 字节\n", .{@sizeOf(Token)});
    try w.interface.print("CValue 大小: {} 字节\n", .{@sizeOf(CValue)});
    try w.interface.print("?usize 大小: {} 字节\n", .{@sizeOf(?usize)});
}
