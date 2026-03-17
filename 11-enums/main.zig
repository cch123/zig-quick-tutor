const std = @import("std");

// === 基本枚举 ===
const Direction = enum {
    north,
    south,
    east,
    west,
};

// === 指定底层类型 ===
const HttpStatus = enum(u16) {
    ok = 200,
    not_found = 404,
    internal_error = 500,
};

// === 带方法的枚举 ===
const Season = enum {
    spring,
    summer,
    autumn,
    winter,

    pub fn isWarm(self: Season) bool {
        return switch (self) {
            .spring, .summer => true,
            .autumn, .winter => false,
        };
    }

    pub fn next(self: Season) Season {
        return switch (self) {
            .spring => .summer,
            .summer => .autumn,
            .autumn => .winter,
            .winter => .spring,
        };
    }

    pub fn name(self: Season) []const u8 {
        return switch (self) {
            .spring => "Spring",
            .summer => "Summer",
            .autumn => "Autumn",
            .winter => "Winter",
        };
    }
};

// === 非穷尽枚举 ===
const FileType = enum(u8) {
    regular = 1,
    directory = 2,
    symlink = 3,
    _, // 允许其他值
};

pub fn main() !void {
    var buf: [4096]u8 = undefined;
    const file = std.fs.File{ .handle = std.posix.STDOUT_FILENO };
    var w = file.writer(&buf);
    defer w.interface.flush() catch {};

    // === 基本使用 ===
    const dir = Direction.north;
    try w.interface.print("方向: {}\n", .{dir});

    // === switch 穷尽性 ===
    const msg = switch (dir) {
        .north => "向北",
        .south => "向南",
        .east => "向东",
        .west => "向西",
    };
    try w.interface.print("方向描述: {s}\n", .{msg});

    // === 枚举方法 ===
    const season = Season.summer;
    try w.interface.print("季节: {s}\n", .{season.name()});
    try w.interface.print("温暖: {}\n", .{season.isWarm()});
    try w.interface.print("下一个: {s}\n", .{season.next().name()});

    // === 枚举与整数转换 ===
    const status = HttpStatus.not_found;
    const status_code = @intFromEnum(status);
    try w.interface.print("HTTP 状态码: {}\n", .{status_code});

    // 整数转枚举
    const from_int: HttpStatus = @enumFromInt(200);
    try w.interface.print("从整数转换: {}\n", .{from_int});

    // === 非穷尽枚举 ===
    const ft: FileType = @enumFromInt(99); // 允许未定义的值
    switch (ft) {
        .regular => try w.interface.print("普通文件\n", .{}),
        .directory => try w.interface.print("目录\n", .{}),
        .symlink => try w.interface.print("符号链接\n", .{}),
        _ => try w.interface.print("未知文件类型: {}\n", .{@intFromEnum(ft)}),
    }

    // === 编译期枚举信息 ===
    const fields = @typeInfo(Direction).@"enum".fields;
    try w.interface.print("\nDirection 枚举有 {} 个值:\n", .{fields.len});
    inline for (fields) |field| {
        try w.interface.print("  - {s} = {}\n", .{ field.name, field.value });
    }

    // === 枚举作为数组索引 ===
    var season_temps = std.EnumArray(Season, i32).initFill(0);
    season_temps.set(.spring, 20);
    season_temps.set(.summer, 35);
    season_temps.set(.autumn, 15);
    season_temps.set(.winter, -5);

    try w.interface.print("\n季节温度:\n", .{});
    inline for (@typeInfo(Season).@"enum".fields) |field| {
        const s: Season = @enumFromInt(field.value);
        try w.interface.print("  {s}: {}°C\n", .{ field.name, season_temps.get(s) });
    }
}
