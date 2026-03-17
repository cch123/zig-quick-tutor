use std::f64::consts::PI;
use std::fmt;

// ============================================================
// 1. Trait 定义
// ============================================================

trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> &str;

    // 默认方法 —— 可以被覆盖
    fn print_area(&self) {
        println!("{} area = {:.2}", self.describe(), self.area());
    }
}

trait Drawable {
    fn draw(&self);
}

// ============================================================
// 2. 显式实现 trait
// ============================================================

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn describe(&self) -> &str {
        "I am a circle"
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn describe(&self) -> &str {
        "I am a rectangle"
    }
}

// ============================================================
// 3. 静态分派 —— impl Trait 参数（零开销，编译期单态化）
// ============================================================

fn print_area_static(shape: &impl Shape) {
    println!("{} area = {:.2}", shape.describe(), shape.area());
}

// ============================================================
// 4. 动态分派 —— dyn Trait（通过 trait object，有虚表开销）
// ============================================================

fn print_area_dynamic(shape: &dyn Shape) {
    println!("{} area = {:.2}", shape.describe(), shape.area());
}

// ============================================================
// 5. Drawable 实现
// ============================================================

struct Star {
    points: u32,
}

impl Drawable for Star {
    fn draw(&self) {
        println!("Drawing a {}-pointed star", self.points);
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Star({})", self.points)
    }
}

struct Line {
    length: f64,
}

impl Drawable for Line {
    fn draw(&self) {
        println!("Drawing a line of length {:.1}", self.length);
    }
}

// ============================================================
// 6. trait bound 组合
// ============================================================

fn draw_and_display(item: &(impl Drawable + fmt::Display)) {
    item.draw();
    println!("Display: {}", item);
}

// 7. 返回 trait object（不同类型统一返回）
fn create_shape(kind: &str) -> Box<dyn Shape> {
    match kind {
        "circle" => Box::new(Circle { radius: 5.0 }),
        _ => Box::new(Rectangle { width: 3.0, height: 4.0 }),
    }
}

fn main() {
    // 1. 静态分派（编译期确定类型，零开销）
    println!("=== static dispatch (impl Trait) ===");
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 3.0, height: 4.0 };
    print_area_static(&c);
    print_area_static(&r);

    // 2. 动态分派（运行时通过虚表调用）
    println!("\n=== dynamic dispatch (dyn Trait) ===");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 5.0, height: 2.0 }),
    ];
    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }

    // 3. 默认方法
    println!("\n=== default method ===");
    c.print_area();

    // 4. trait object 返回值
    println!("\n=== trait object return ===");
    let s1 = create_shape("circle");
    let s2 = create_shape("rect");
    s1.print_area();
    s2.print_area();

    // 5. Drawable 接口
    println!("\n=== Drawable trait ===");
    let drawables: Vec<Box<dyn Drawable>> = vec![
        Box::new(Star { points: 5 }),
        Box::new(Line { length: 10.0 }),
    ];
    for d in &drawables {
        d.draw();
    }

    // 6. trait bound 组合
    println!("\n=== combined trait bounds ===");
    let star = Star { points: 6 };
    draw_and_display(&star);

    // Rust 的 trait 系统：
    // - 静态分派（impl Trait / 泛型）：零开销，编译期单态化
    // - 动态分派（dyn Trait）：通过 trait object（fat pointer = 数据指针 + 虚表指针）
    // - 必须显式 impl，编译器检查一致性
}
