use std::f64::consts::PI;

// Rust 的 enum 可以携带数据（类似 Zig 的 tagged union）
enum Shape {
    Circle(f64),                // 半径
    Rectangle { w: f64, h: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle { w, h } => w * h,
            Shape::Triangle { base, height } => base * height / 2.0,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "圆形",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
}

fn main() {
    // === 基本 match ===
    let x: i32 = 3;

    // match 是表达式，可以赋值
    let label = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 | 5 => "four or five", // 多值匹配用 |
        6..=10 => "six to ten",  // 范围匹配（包含两端）
        _ => "other",            // _ 是通配符，类似 default
    };
    println!("x = {}, label = {}", x, label);

    // === match 用于范围和守卫 ===
    let grade: u32 = 85;
    match grade {
        90..=100 => println!("优秀 (A)"),
        80..=89 => println!("良好 (B)"),
        70..=79 => println!("中等 (C)"),
        60..=69 => println!("及格 (D)"),
        _ => println!("不及格 (F)"),
    }

    // === enum 解构匹配 ===
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { w: 4.0, h: 6.0 },
        Shape::Triangle { base: 3.0, height: 8.0 },
    ];

    for s in &shapes {
        println!("{} 的面积 = {:.2}", s.name(), s.area());
    }

    // === 模式守卫（match guard） ===
    let num = 15;
    let description = match num {
        n if n < 0 => "负数",
        0 => "零",
        n if n % 2 == 0 => "正偶数",
        _ => "正奇数",
    };
    println!("{} 是{}", num, description);

    // === 元组解构 ===
    let point = (3, -5);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("x 轴上, x={}", x),
        (0, y) => println!("y 轴上, y={}", y),
        (x, y) => println!("其他位置: ({}, {})", x, y),
    }

    // === bool match ===
    let flag = true;
    let msg = match flag {
        true => "开",
        false => "关",
    };
    println!("开关状态: {}", msg);
}
