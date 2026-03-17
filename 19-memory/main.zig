const std = @import("std");
const print = std.debug.print;

// 1. Allocator 作为参数模式 —— Zig 的核心设计
fn createMessage(allocator: std.mem.Allocator, name: []const u8) ![]u8 {
    const msg = try std.fmt.allocPrint(allocator, "Hello, {s}!", .{name});
    return msg;
}

// 2. 使用分配器的数据结构
fn sumOfList(allocator: std.mem.Allocator, n: usize) !u64 {
    var list = std.ArrayList(u64){};
    defer list.deinit(allocator);

    for (0..n) |i| {
        try list.append(allocator, i + 1);
    }

    var sum: u64 = 0;
    for (list.items) |v| {
        sum += v;
    }
    return sum;
}

pub fn main() !void {
    // ============================================================
    // 1. GeneralPurposeAllocator —— 通用分配器，带安全检测
    // ============================================================
    print("=== GeneralPurposeAllocator ===\n", .{});
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const check = gpa.deinit();
        if (check == .leak) {
            print("Memory leak detected!\n", .{});
        } else {
            print("GPA: no leaks detected\n", .{});
        }
    }
    const gpa_alloc = gpa.allocator();

    // 分配单个切片
    const data = try gpa_alloc.alloc(u8, 10);
    defer gpa_alloc.free(data);
    @memset(data, 'A');
    print("data: {s}\n", .{data});

    // 分配器作为参数传递
    const msg = try createMessage(gpa_alloc, "Zig");
    defer gpa_alloc.free(msg);
    print("message: {s}\n", .{msg});

    // ============================================================
    // 2. ArenaAllocator —— 批量分配，一次性释放
    // ============================================================
    print("\n=== ArenaAllocator ===\n", .{});
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit(); // 一次性释放所有分配的内存

    const arena_alloc = arena.allocator();

    // 在 arena 中分配多个对象，无需单独释放
    const buf1 = try arena_alloc.alloc(u8, 100);
    const buf2 = try arena_alloc.alloc(u8, 200);
    const buf3 = try arena_alloc.alloc(u8, 50);
    _ = buf2;
    _ = buf3;
    @memcpy(buf1[0..5], "Arena");
    print("arena buf1: {s}\n", .{buf1[0..5]});
    // 不需要逐个 free —— arena.deinit() 会释放全部

    // ============================================================
    // 3. FixedBufferAllocator —— 栈上缓冲区，零系统调用
    // ============================================================
    print("\n=== FixedBufferAllocator ===\n", .{});
    var stack_buf: [256]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&stack_buf);
    const fba_alloc = fba.allocator();

    const small1 = try fba_alloc.alloc(u8, 10);
    const small2 = try fba_alloc.alloc(u8, 20);
    @memset(small1, 'X');
    @memset(small2, 'Y');
    print("fixed buf1: {s}\n", .{small1});
    print("fixed buf2: {s}\n", .{small2});
    // FixedBufferAllocator 从预分配的栈缓冲区中切分，无堆分配

    // ============================================================
    // 4. page_allocator —— 直接系统调用
    // ============================================================
    print("\n=== page_allocator ===\n", .{});
    const page_data = try std.heap.page_allocator.alloc(u8, 64);
    defer std.heap.page_allocator.free(page_data);
    @memset(page_data, 'P');
    print("page data[0..5]: {s}\n", .{page_data[0..5]});

    // ============================================================
    // 5. @memcpy 和 @memset
    // ============================================================
    print("\n=== @memcpy / @memset ===\n", .{});
    var src = [_]u8{ 'H', 'e', 'l', 'l', 'o' };
    var dst: [5]u8 = undefined;
    @memcpy(&dst, &src);
    print("copied: {s}\n", .{dst});

    @memset(&dst, 0);
    print("zeroed: {any}\n", .{dst});

    // ============================================================
    // 6. 分配器作为参数 —— 库代码的标准模式
    // ============================================================
    print("\n=== allocator as parameter ===\n", .{});
    const sum = try sumOfList(gpa_alloc, 100);
    print("sum(1..100) = {}\n", .{sum});

    // Zig 的内存管理哲学：
    // - 没有隐藏的堆分配
    // - 分配器作为参数传递，调用者决定内存策略
    // - debug 模式下 GPA 自动检测内存泄漏和越界访问
    // - defer 确保释放不会被遗忘
}
