fn main() {
    // === 字符串基础 ===
    // &str: 字符串切片引用（借用），不可变
    let hello: &str = "Hello, Rust!";
    println!("字符串: {}", hello);
    println!("长度(字节): {}", hello.len());

    // String: 堆分配、可增长、拥有所有权
    let owned: String = String::from("Hello, Owned!");
    println!("拥有的字符串: {}", owned);

    // 访问单个字节
    println!("第一个字节: '{}' (0x{:x})", hello.as_bytes()[0] as char, hello.as_bytes()[0]);

    // === 字符串拼接 ===
    // format! 宏
    let greeting = format!("{}, {}!", "Hello", "World");
    println!("format! 拼接: {}", greeting);

    // String::push_str
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("push_str 拼接: {}", s);

    // + 运算符（消耗左操作数的所有权）
    let a = String::from("Hello");
    let b = ", World!";
    let c = a + b; // a 被移动，不能再使用
    println!("+ 拼接: {}", c);

    // join
    let parts = vec!["Hello", ", ", "World!"];
    let joined = parts.join("");
    println!("join 拼接: {}", joined);

    // === 字符串格式化 ===
    let formatted = format!("name={}, age={}", "Alice", 30);
    println!("格式化: {}", formatted);

    // 各种格式化选项
    println!("十六进制: 0x{:x}", 255);
    println!("补零: {:05}", 42);
    println!("左对齐: |{:<10}|", "left");
    println!("右对齐: |{:>10}|", "right");

    // === 多行字符串 ===
    let multiline = "这是第一行\n\
                     这是第二行\n\
                     这是第三行";
    println!("多行字符串:\n{}", multiline);

    // 原始字符串
    let raw = r#"这里可以包含 "引号" 和 \反斜杠\"#;
    println!("原始字符串: {}", raw);

    // === 字符串比较 ===
    let x = "hello";
    let y = "hello";
    let z = "world";
    println!("x == y: {}", x == y);
    println!("x == z: {}", x == z);

    // === 字符串查找 ===
    let haystack = "Hello, World!";
    if let Some(index) = haystack.find("World") {
        println!("找到 'World' 在位置: {}", index);
    }

    // === 遍历字节 ===
    print!("逐字节: ");
    for byte in "Rust".bytes() {
        print!("{} ", byte as char);
    }
    println!();

    // === 遍历字符（Unicode 码点） ===
    let chinese = "你好世界";
    println!("UTF-8 字节长度: {}", chinese.len());
    println!("UTF-8 字符数: {}", chinese.chars().count());

    print!("逐字符: ");
    for ch in chinese.chars() {
        print!("{} ", ch);
    }
    println!();

    // === String 与 &str 转换 ===
    let string: String = String::from("hello");
    let slice: &str = &string;         // String -> &str（自动解引用）
    let owned: String = slice.to_string(); // &str -> String
    println!("slice: {}, owned: {}", slice, owned);

    // === 字符串切片（注意：必须在字符边界） ===
    let s = "Hello, 世界";
    let hello_part = &s[0..5]; // OK: ASCII 范围
    println!("切片: {}", hello_part);
    // let bad = &s[0..8]; // panic! 不在字符边界
}
