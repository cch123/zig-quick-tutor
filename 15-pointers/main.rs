fn increment(val: &mut i32) {
    *val += 1; // 通过可变引用修改值
}

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    // 1. 不可变引用 &T（共享借用）
    let x = 42;
    let r = &x;
    println!("x = {}, *r = {}", x, *r);
    // Rust 会自动解引用，所以通常不需要写 *r
    println!("r (auto-deref) = {}", r);

    // 2. 可变引用 &mut T（独占借用）
    let mut y = 100;
    {
        let r_mut = &mut y;
        *r_mut += 1;
        println!("After mutation: {}", r_mut);
    } // r_mut 的借用在此结束
    println!("y = {}", y); // 101

    // 3. 通过可变引用传参
    let mut z = 50;
    increment(&mut z);
    println!("After increment: z = {}", z); // 51

    // 4. 借用规则：同一时刻只能有一个可变引用 OR 多个不可变引用
    let mut data = 42;
    let r1 = &data;
    let r2 = &data; // OK：多个不可变引用
    println!("r1={}, r2={}", r1, r2);
    // r1 和 r2 不再使用后，可以创建可变引用
    let r3 = &mut data;
    *r3 = 99;
    println!("After mut: {}", data);

    // 5. Box<T> - 堆分配的拥有型指针
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed); // 自动解引用
    println!("Boxed size on stack: {} bytes", std::mem::size_of_val(&boxed));

    // Box 常用于递归类型
    #[derive(Debug)]
    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("List: {:?}", list);

    // 6. 切片引用 &[T]（类似 Zig 的 []const T）
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // [20, 30, 40]
    println!("Slice: {:?}, sum = {}", slice, sum(slice));

    // 7. 原始指针（仅在 unsafe 中解引用）
    let val = 42i32;
    let raw_ptr: *const i32 = &val;
    let raw_mut: *mut i32 = &val as *const i32 as *mut i32;
    unsafe {
        println!("Raw pointer: {}", *raw_ptr);
        // 可以做指针算术
        let arr2 = [100i32, 200, 300];
        let p = arr2.as_ptr();
        println!("arr2[0] = {}", *p);
        println!("arr2[2] = {}", *p.add(2)); // 指针算术
    }
    let _ = raw_mut; // 避免 unused 警告

    // 8. 自动解引用（Deref trait）
    let s = String::from("hello");
    // String 自动解引用为 &str
    fn takes_str(s: &str) {
        println!("Got: {}", s);
    }
    takes_str(&s); // String -> &str 自动转换

    // 9. 字节数据重解释（类似 C 的类型双关）
    let bytes: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    let int_val: u32 = u32::from_le_bytes(bytes);
    println!("Bytes as u32: {}", int_val);
}
