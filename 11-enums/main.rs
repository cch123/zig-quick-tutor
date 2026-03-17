use std::fmt;

// === 基本枚举 ===
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

// === 指定底层类型 ===
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}

// === 带方法的枚举 ===
#[derive(Debug, Clone, Copy, PartialEq)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    fn is_warm(&self) -> bool {
        matches!(self, Season::Spring | Season::Summer)
    }

    fn next(&self) -> Season {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }

    fn name(&self) -> &str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
}

// === Rust 枚举的真正威力：关联数据 ===
#[derive(Debug)]
enum Shape {
    Circle(f64),                      // 关联一个 f64
    Rectangle { width: f64, height: f64 }, // 关联命名字段
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle { .. } => "Triangle",
        }
    }
}

// === Option 和 Result 就是枚举 ===
// enum Option<T> { Some(T), None }
// enum Result<T, E> { Ok(T), Err(E) }

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // === 基本使用 ===
    let dir = Direction::North;
    println!("方向: {}", dir);

    // === match 穷尽性（必须覆盖所有变体） ===
    let msg = match dir {
        Direction::North => "向北",
        Direction::South => "向南",
        Direction::East => "向东",
        Direction::West => "向西",
    };
    println!("方向描述: {}", msg);

    // === 枚举方法 ===
    let season = Season::Summer;
    println!("季节: {}", season.name());
    println!("温暖: {}", season.is_warm());
    println!("下一个: {}", season.next().name());

    // === 枚举与整数转换 ===
    let status = HttpStatus::NotFound;
    let code = status as u16;
    println!("HTTP 状态码: {}", code);

    // === 关联数据（Rust 独有的强大功能） ===
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 8.0 },
    ];

    println!("\n形状面积:");
    for shape in &shapes {
        println!("  {}: {:.2}", shape.name(), shape.area());
    }

    // === if let 语法糖 ===
    let maybe_value: Option<i32> = Some(42);
    if let Some(v) = maybe_value {
        println!("\n有值: {}", v);
    }

    // === Result 处理 ===
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("错误: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {:.2}", result),
        Err(e) => println!("错误: {}", e),
    }

    // === 迭代所有枚举值（需要手动实现或使用 strum crate） ===
    let seasons = [Season::Spring, Season::Summer, Season::Autumn, Season::Winter];
    println!("\n季节温度:");
    let temps = [20, 35, 15, -5];
    for (season, temp) in seasons.iter().zip(temps.iter()) {
        println!("  {}: {}°C", season.name(), temp);
    }
}
