const std = @import("std");

pub fn main() !void {
    var buf: [4096]u8 = undefined;
    const file = std.fs.File{ .handle = std.posix.STDOUT_FILENO };
    var w = file.writer(&buf);
    defer w.interface.flush() catch {};

    // === 字符串基础 ===
    // 字符串字面量类型是 *const [N:0]u8，可以自动转为 []const u8
    const hello: []const u8 = "Hello, Zig!";
    try w.interface.print("字符串: {s}\n", .{hello});
    try w.interface.print("长度: {}\n", .{hello.len});

    // 访问单个字节
    try w.interface.print("第一个字节: '{c}' (0x{x})\n", .{ hello[0], hello[0] });

    // === 编译期字符串拼接 ===
    // ++ 运算符只能用于编译期已知的值
    const first = "Hello";
    const second = ", World!";
    const greeting = first ++ second;
    try w.interface.print("编译期拼接: {s}\n", .{greeting});

    // 编译期重复
    const repeated = "ha" ** 3;
    try w.interface.print("重复: {s}\n", .{repeated}); // "hahaha"

    // === 运行时字符串拼接 ===
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const parts = [_][]const u8{ "Hello", ", ", "World!" };
    const joined = try std.mem.concat(allocator, u8, &parts);
    defer allocator.free(joined);
    try w.interface.print("运行时拼接: {s}\n", .{joined});

    // === 字符串格式化 ===
    var fmt_buf: [256]u8 = undefined;
    const formatted = try std.fmt.bufPrint(&fmt_buf, "name={s}, age={d}", .{ "Alice", @as(u32, 30) });
    try w.interface.print("格式化: {s}\n", .{formatted});

    // 使用分配器的格式化
    const alloc_fmt = try std.fmt.allocPrint(allocator, "{s} is {d} years old", .{ "Bob", @as(u32, 25) });
    defer allocator.free(alloc_fmt);
    try w.interface.print("堆分配格式化: {s}\n", .{alloc_fmt});

    // === 多行字符串 ===
    const multiline =
        \\这是第一行
        \\这是第二行
        \\这是第三行
    ;
    try w.interface.print("多行字符串:\n{s}\n", .{multiline});

    // === 字符串比较 ===
    const a: []const u8 = "hello";
    const b: []const u8 = "hello";
    const c: []const u8 = "world";
    try w.interface.print("a == b: {}\n", .{std.mem.eql(u8, a, b)});
    try w.interface.print("a == c: {}\n", .{std.mem.eql(u8, a, c)});

    // === 字符串查找 ===
    const haystack = "Hello, World!";
    if (std.mem.indexOf(u8, haystack, "World")) |index| {
        try w.interface.print("找到 'World' 在位置: {}\n", .{index});
    }

    // === 遍历字节 ===
    try w.interface.print("逐字节: ", .{});
    for ("Zig") |byte| {
        try w.interface.print("{c} ", .{byte});
    }
    try w.interface.print("\n", .{});

    // === UTF-8 处理 ===
    const chinese = "你好世界";
    try w.interface.print("UTF-8 字节长度: {}\n", .{chinese.len});

    var utf8_len: usize = 0;
    var view = std.unicode.Utf8View.initUnchecked(chinese);
    var iter = view.iterator();
    while (iter.nextCodepoint()) |_| {
        utf8_len += 1;
    }
    try w.interface.print("UTF-8 字符数: {}\n", .{utf8_len});
}
