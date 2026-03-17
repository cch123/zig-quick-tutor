fn main() {
    // 整数类型
    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = 42;
    let d: u64 = 18_446_744_073_709_551_615;

    println!("i8: {}", a);
    println!("u8: {}", b);
    println!("i32: {}", c);
    println!("u64: {}", d);

    // 浮点类型
    let f1: f32 = 3.14;
    let f2: f64 = 2.718281828;
    println!("f32: {:.2}", f1);
    println!("f64: {:.9}", f2);

    // 布尔类型
    let yes: bool = true;
    let no: bool = false;
    println!("bool: {} {}", yes, no);

    // 字符 (char = Unicode scalar value, 4 bytes)
    let ch: char = 'A';
    println!("char: {}", ch);
    println!("char as int: {}", ch as u32);

    // 整数字面量默认推断为 i32
    let big: u64 = 1_000_000_000_000_000;
    let result: u64 = big * 2;
    println!("integer result: {}", result);

    // 类型推断
    let inferred = 100i32;
    println!("inferred: {}", inferred);
}
