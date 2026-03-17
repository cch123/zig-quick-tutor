use std::f64::consts::PI;

// === Rust 的 enum 就是 tagged union ===
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
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

// === Token：更复杂的 tagged union ===
#[derive(Debug)]
enum Token {
    Number(f64),
    Str(String),
    Plus,
    Minus,
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Str(s) => write!(f, "String(\"{}\")", s),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

// === Option<T> 和 Result<T, E> 是标准库的 tagged union ===
fn find_index(data: &str, target: char) -> Option<usize> {
    data.find(target)
}

#[derive(Debug)]
enum ParseError {
    InvalidCharacter(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidCharacter(c) => write!(f, "invalid character: {}", c),
        }
    }
}

fn parse_digit(c: char) -> Result<u8, ParseError> {
    if c.is_ascii_digit() {
        Ok(c as u8 - b'0')
    } else {
        Err(ParseError::InvalidCharacter(c))
    }
}

// === 递归 tagged union（链表） ===
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        List::Nil
    }

    fn push(self, val: i32) -> Self {
        List::Cons(val, Box::new(self))
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = self;
        while let List::Cons(val, next) = current {
            result.push(*val);
            current = next;
        }
        result
    }
}

fn main() {
    // === Tagged union 基本使用 ===
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 8.0 },
    ];

    println!("=== 形状面积 ===");
    for shape in &shapes {
        println!("  {}: {:.2}", shape.name(), shape.area());
    }

    // === match 解构（必须穷尽） ===
    println!("\n=== match 解构 ===");
    for shape in &shapes {
        match shape {
            Shape::Circle(r) => println!("  圆的半径: {}", r),
            Shape::Rectangle { width, height } => println!("  矩形: {}x{}", width, height),
            Shape::Triangle { base, height } => println!("  三角形: base={}, h={}", base, height),
        }
    }

    // === Token 示例 ===
    let tokens = vec![
        Token::Number(42.0),
        Token::Plus,
        Token::Number(8.0),
        Token::Eof,
    ];

    println!("\n=== 词法标记 ===");
    for token in &tokens {
        println!("  {}", token);
    }

    // === Option<T> ===
    println!("\n=== Option<T> ===");
    let data = "Hello, World!";

    if let Some(index) = find_index(data, 'W') {
        println!("找到 'W' 在位置: {}", index);
    }

    if let Some(index) = find_index(data, 'Z') {
        println!("找到 'Z' 在位置: {}", index);
    } else {
        println!("未找到 'Z'");
    }

    // unwrap_or 提供默认值（类似 Zig 的 orelse）
    let idx = find_index(data, 'Z').unwrap_or(0);
    println!("unwrap_or 默认值: {}", idx);

    // === Result<T, E> ===
    println!("\n=== Result<T, E> ===");
    match parse_digit('5') {
        Ok(digit) => println!("解析 '5': {}", digit),
        Err(e) => println!("错误: {}", e),
    }

    match parse_digit('x') {
        Ok(digit) => println!("解析 'x': {}", digit),
        Err(e) => println!("解析 'x' 错误: {}", e),
    }

    // ? 操作符（类似 Zig 的 try）在返回 Result 的函数中使用

    // === 递归 tagged union ===
    println!("\n=== 递归 tagged union（链表） ===");
    let list = List::new().push(3).push(2).push(1);
    println!("链表: {:?}", list.to_vec());

    // === 内存大小 ===
    println!("\n=== 内存大小 ===");
    println!("Shape 大小: {} 字节", std::mem::size_of::<Shape>());
    println!("Token 大小: {} 字节", std::mem::size_of::<Token>());
    println!("Option<usize> 大小: {} 字节", std::mem::size_of::<Option<usize>>());
    println!("Result<u8, ParseError> 大小: {} 字节", std::mem::size_of::<Result<u8, ParseError>>());
}
