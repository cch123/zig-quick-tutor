const std = @import("std");

// === 基本结构体 ===
const Point = struct {
    x: f64,
    y: f64,

    // 方法：显式 self 参数
    pub fn distanceTo(self: Point, other: Point) f64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        return @sqrt(dx * dx + dy * dy);
    }

    // 构造函数惯用法
    pub fn init(x: f64, y: f64) Point {
        return .{ .x = x, .y = y };
    }

    // 自定义格式化（Zig 0.15 新签名）
    pub fn format(self: Point, writer: anytype) !void {
        try writer.print("Point({d:.1}, {d:.1})", .{ self.x, self.y });
    }
};

// === 带默认值的结构体 ===
const Config = struct {
    host: []const u8 = "localhost",
    port: u16 = 8080,
    debug: bool = false,
};

// === packed struct：精确控制内存布局 ===
const Flags = packed struct {
    read: bool,
    write: bool,
    execute: bool,
    _reserved: u5 = 0,
};

// === extern struct：C 兼容布局 ===
const CPoint = extern struct {
    x: f64,
    y: f64,
};

// === 带指针参数的方法（可变 self）===
const Counter = struct {
    count: u32 = 0,

    pub fn increment(self: *Counter) void {
        self.count += 1;
    }

    pub fn getCount(self: Counter) u32 {
        return self.count;
    }
};

// === 泛型结构体 ===
fn Pair(comptime T: type) type {
    return struct {
        first: T,
        second: T,

        const Self = @This();

        pub fn init(first: T, second: T) Self {
            return .{ .first = first, .second = second };
        }

        pub fn swap(self: Self) Self {
            return .{ .first = self.second, .second = self.first };
        }
    };
}

pub fn main() !void {
    var buf: [4096]u8 = undefined;
    const file = std.fs.File{ .handle = std.posix.STDOUT_FILENO };
    var w = file.writer(&buf);
    defer w.interface.flush() catch {};

    // === 基本使用 ===
    const p1 = Point.init(0, 0);
    const p2 = Point{ .x = 3, .y = 4 };
    try w.interface.print("{f} 到 {f} 的距离: {d:.2}\n", .{ p1, p2, p1.distanceTo(p2) });

    // === 默认值 ===
    const default_cfg = Config{};
    const custom_cfg = Config{ .port = 9090, .debug = true };
    try w.interface.print("默认配置: {s}:{}\n", .{ default_cfg.host, default_cfg.port });
    try w.interface.print("自定义配置: {s}:{} debug={}\n", .{ custom_cfg.host, custom_cfg.port, custom_cfg.debug });

    // === packed struct ===
    const flags = Flags{ .read = true, .write = true, .execute = false };
    const as_byte: u8 = @bitCast(flags);
    try w.interface.print("Flags 位表示: 0b{b:0>8}\n", .{as_byte});
    try w.interface.print("Flags 大小: {} 字节\n", .{@sizeOf(Flags)});

    // === 可变方法 ===
    var counter = Counter{};
    counter.increment();
    counter.increment();
    counter.increment();
    try w.interface.print("计数器: {}\n", .{counter.getCount()});

    // === 匿名结构体 ===
    const anon = .{ .name = "Zig", .version = @as(u32, 13) };
    try w.interface.print("匿名结构体: name={s}, version={}\n", .{ anon.name, anon.version });

    // === 元组 ===
    const tuple = .{ @as(i32, 42), "hello", true };
    try w.interface.print("元组: {}, {s}, {}\n", .{ tuple[0], tuple[1], tuple[2] });

    // === 泛型结构体 ===
    const int_pair = Pair(i32).init(10, 20);
    const swapped = int_pair.swap();
    try w.interface.print("原始: ({}, {})\n", .{ int_pair.first, int_pair.second });
    try w.interface.print("交换: ({}, {})\n", .{ swapped.first, swapped.second });

    // === 结构体大小 ===
    try w.interface.print("\n--- 内存布局 ---\n", .{});
    try w.interface.print("Point 大小: {} 字节\n", .{@sizeOf(Point)});
    try w.interface.print("CPoint 大小: {} 字节\n", .{@sizeOf(CPoint)});
    try w.interface.print("Config 大小: {} 字节\n", .{@sizeOf(Config)});
}
