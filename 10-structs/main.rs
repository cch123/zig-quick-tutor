use std::fmt;
use std::mem;

// === 基本结构体 ===
struct Point {
    x: f64,
    y: f64,
}

// impl 块定义方法
impl Point {
    // 关联函数（类似构造函数）
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // 方法：&self 借用
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({:.1}, {:.1})", self.x, self.y)
    }
}

// === 带 Default trait 的结构体 ===
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
            debug: false,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config{{ host={}, port={}, debug={} }}", self.host, self.port, self.debug)
    }
}

// === 可变方法 ===
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> u32 {
        self.count
    }
}

// === 元组结构体 ===
struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.0, self.1, self.2)
    }
}

// === 单元结构体 ===
struct Unit;

// === 泛型结构体 ===
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Clone> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn swap(&self) -> Self {
        Pair {
            first: self.second.clone(),
            second: self.first.clone(),
        }
    }
}

// === C 兼容布局 ===
#[repr(C)]
struct CPoint {
    x: f64,
    y: f64,
}

// === packed 布局 ===
#[repr(packed)]
struct PackedData {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    // === 基本使用 ===
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("{} 到 {} 的距离: {:.2}", p1, p2, p1.distance_to(&p2));

    // === Default trait ===
    let default_cfg = Config::default();
    let custom_cfg = Config {
        port: 9090,
        debug: true,
        ..Config::default() // 用 default 填充其余字段
    };
    println!("默认配置: {}", default_cfg);
    println!("自定义配置: {}", custom_cfg);

    // === 可变方法 ===
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("计数器: {}", counter.get_count());

    // === 元组结构体 ===
    let red = Color(255, 0, 0);
    println!("颜色: {}", red);

    // === 解构 ===
    let Point { x, y } = Point::new(10.0, 20.0);
    println!("解构: x={}, y={}", x, y);

    // === 泛型结构体 ===
    let int_pair = Pair::new(10, 20);
    let swapped = int_pair.swap();
    println!("原始: ({}, {})", int_pair.first, int_pair.second);
    println!("交换: ({}, {})", swapped.first, swapped.second);

    // === 所有权与方法 ===
    // &self: 借用，调用后仍可使用
    // &mut self: 可变借用
    // self: 消耗所有权，调用后不可再使用

    // === 结构体大小 ===
    println!("\n--- 内存布局 ---");
    println!("Point 大小: {} 字节", mem::size_of::<Point>());
    println!("CPoint 大小: {} 字节", mem::size_of::<CPoint>());
    println!("Config 大小: {} 字节", mem::size_of::<Config>());
    println!("PackedData 大小: {} 字节 (未 packed 时可能更大)", mem::size_of::<PackedData>());
    println!("Unit 大小: {} 字节", mem::size_of::<Unit>());
}
