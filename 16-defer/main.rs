// Rust 没有 defer 关键字，使用 RAII (Drop trait) 进行资源管理

use std::fmt;

// 1. 实现 Drop trait 的资源类型
struct Logger {
    name: String,
}

impl Logger {
    fn new(name: &str) -> Self {
        println!("[{}] opened", name);
        Logger {
            name: name.to_string(),
        }
    }

    fn write(&self, msg: &str) {
        println!("[{}] {}", self.name, msg);
    }
}

// Drop trait = 析构函数，变量离开作用域时自动调用
impl Drop for Logger {
    fn drop(&mut self) {
        println!("[{}] closed (Drop)", self.name);
    }
}

// 2. 自定义 Defer 结构体模拟 defer 行为
struct Defer<F: FnOnce()> {
    f: Option<F>,
}

impl<F: FnOnce()> Defer<F> {
    fn new(f: F) -> Self {
        Defer { f: Some(f) }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}

// 3. 一个拥有资源的结构体
struct Connection {
    id: u32,
    connected: bool,
}

impl Connection {
    fn new(id: u32) -> Self {
        println!("Connection {} established", id);
        Connection {
            id,
            connected: true,
        }
    }

    fn query(&self, sql: &str) -> String {
        format!("Result from conn {}: {}", self.id, sql)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.connected {
            println!("Connection {} closed (Drop)", self.id);
            self.connected = false;
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection(id={})", self.id)
    }
}

fn main() {
    // 1. 基本 RAII - Drop 自动调用
    println!("--- basic RAII ---");
    {
        let logger = Logger::new("basic");
        logger.write("doing work...");
        logger.write("more work...");
        // logger 在此自动 drop
    }
    println!("After block");

    // 2. 多个资源按声明的逆序释放（类似 defer LIFO）
    println!("\n--- multiple drops (reverse order) ---");
    {
        let _l1 = Logger::new("first");
        let _l2 = Logger::new("second");
        let _l3 = Logger::new("third");
        println!("main body");
        // drop 顺序：third, second, first
    }

    // 3. 作用域级释放（类似 Zig 的作用域 defer）
    println!("\n--- scoped drop in loop ---");
    for i in 0..3 {
        let logger = Logger::new(&format!("loop-{}", i));
        logger.write("iteration work");
        // 每次迭代结束时自动 drop（和 Zig 一样安全）
    }
    println!("After loop (all loggers already closed)");

    // 4. std::mem::drop() 提前释放
    println!("\n--- explicit drop ---");
    {
        let logger = Logger::new("early");
        logger.write("some work");
        drop(logger); // 提前释放
        println!("Logger already dropped here");
        // logger.write("error!"); // 编译错误！已移动
    }

    // 5. 模拟 defer（使用闭包 + Drop）
    println!("\n--- simulated defer ---");
    {
        let _d1 = Defer::new(|| println!("deferred action 1 (last)"));
        let _d2 = Defer::new(|| println!("deferred action 2 (first)"));
        println!("doing work with deferred cleanup");
        // drop 顺序：d2, d1
    }

    // 6. Connection 示例 - 实际的资源管理
    println!("\n--- connection RAII ---");
    {
        let conn = Connection::new(1);
        println!("{}", conn.query("SELECT * FROM users"));
        println!("{}", conn.query("SELECT * FROM orders"));
        // conn 在此自动关闭
    }

    // 7. 在循环中使用 RAII（完全安全）
    println!("\n--- connections in loop ---");
    for i in 0..3 {
        let conn = Connection::new(i);
        println!("{}", conn.query(&format!("query {}", i)));
        // 每次迭代结束自动关闭连接
    }

    // 8. Rust 没有 errdefer，但 Drop 在 panic 时也会执行
    println!("\n--- Drop on panic (catch_unwind) ---");
    let result = std::panic::catch_unwind(|| {
        let _logger = Logger::new("panic-test");
        println!("About to panic...");
        panic!("something went wrong");
        // logger 的 Drop 仍然会执行！
    });
    match result {
        Ok(_) => println!("No panic"),
        Err(_) => println!("Recovered from panic (Drop still ran)"),
    }
}
