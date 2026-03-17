const std = @import("std");
const print = std.debug.print;

// ============================================================
// 方式一：comptime duck typing（静态分派）
// ============================================================

// anytype 参数 —— 编译期检查类型是否有 area() 方法
fn printArea(shape: anytype) void {
    // 编译期检查：如果类型没有 area() 方法，编译失败
    const area = shape.area();
    const name = @typeName(@TypeOf(shape));
    print("{s} area = {d:.2}\n", .{ name, area });
}

// ============================================================
// 方式二：tagged union（有限多态）
// ============================================================

const Shape = union(enum) {
    circle: Circle,
    rectangle: Rectangle,

    fn area(self: Shape) f64 {
        return switch (self) {
            .circle => |c| c.area(),
            .rectangle => |r| r.area(),
        };
    }

    fn describe(self: Shape) void {
        switch (self) {
            .circle => print("I am a circle\n", .{}),
            .rectangle => print("I am a rectangle\n", .{}),
        }
    }
};

const Circle = struct {
    radius: f64,

    fn area(self: Circle) f64 {
        return std.math.pi * self.radius * self.radius;
    }
};

const Rectangle = struct {
    width: f64,
    height: f64,

    fn area(self: Rectangle) f64 {
        return self.width * self.height;
    }
};

// ============================================================
// 方式三：手动虚表 / fat pointer（动态分派）
// 这是 std.mem.Allocator 等标准库类型使用的模式
// ============================================================

const Drawable = struct {
    // 类型擦除指针 —— 指向具体类型的实例
    ptr: *anyopaque,
    // 虚表 —— 函数指针
    drawFn: *const fn (ptr: *anyopaque) void,

    fn draw(self: Drawable) void {
        self.drawFn(self.ptr);
    }

    // 工厂函数：从任意实现了 draw 的类型创建 Drawable
    fn init(ptr: anytype) Drawable {
        const T = @TypeOf(ptr);
        const impl = struct {
            fn draw(raw: *anyopaque) void {
                const self: T = @ptrCast(@alignCast(raw));
                self.draw();
            }
        };
        return .{
            .ptr = ptr,
            .drawFn = impl.draw,
        };
    }
};

const Star = struct {
    points: u32,

    fn draw(self: *const Star) void {
        print("Drawing a {}-pointed star\n", .{self.points});
    }
};

const Line = struct {
    length: f64,

    fn draw(self: *const Line) void {
        print("Drawing a line of length {d:.1}\n", .{self.length});
    }
};

pub fn main() void {
    // 方式一：comptime duck typing（静态分派）
    print("=== comptime duck typing ===\n", .{});
    const c = Circle{ .radius = 5.0 };
    const r = Rectangle{ .width = 3.0, .height = 4.0 };
    printArea(c);
    printArea(r);

    // 方式二：tagged union
    print("\n=== tagged union ===\n", .{});
    const shapes = [_]Shape{
        Shape{ .circle = Circle{ .radius = 3.0 } },
        Shape{ .rectangle = Rectangle{ .width = 5.0, .height = 2.0 } },
    };
    for (shapes) |shape| {
        shape.describe();
        print("  area = {d:.2}\n", .{shape.area()});
    }

    // 方式三：手动虚表（动态分派）
    print("\n=== manual vtable (fat pointer) ===\n", .{});
    var star = Star{ .points = 5 };
    var line = Line{ .length = 10.0 };

    // 不同类型通过统一接口调用
    const drawables = [_]Drawable{
        Drawable.init(&star),
        Drawable.init(&line),
    };
    for (drawables) |d| {
        d.draw();
    }
}
