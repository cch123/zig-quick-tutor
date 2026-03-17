use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

// ============================================================
// 示例 1: TCP Echo 服务器 + 客户端
// ============================================================

fn echo_server_demo() {
    println!("=== TCP Echo 服务器 ===");

    let listener = TcpListener::bind("127.0.0.1:8080").expect("绑定失败");
    println!("服务器监听在 127.0.0.1:8080");

    // 在子线程中启动客户端
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));

        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("连接失败");
        println!("客户端已连接");

        let message = b"Hello, Rust Networking!\n";
        stream.write_all(message).expect("发送失败");
        print!("发送: {}", String::from_utf8_lossy(message));

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).expect("读取失败");
        print!("收到回显: {}\n", String::from_utf8_lossy(&buf[..n]));
    });

    // 服务器接受一个连接
    let (mut stream, addr) = listener.accept().expect("接受连接失败");
    println!("客户端连接: {}", addr);

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).expect("读取失败");
    print!("服务端收到: {}", String::from_utf8_lossy(&buf[..n]));
    stream.write_all(&buf[..n]).expect("写入失败");

    thread::sleep(Duration::from_millis(100));
    println!();
}

// ============================================================
// 示例 2: 请求-响应服务器
// ============================================================

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return,
    };

    let data = String::from_utf8_lossy(&buf[..n]);
    let parts: Vec<&str> = data.splitn(2, ':').collect();

    if parts.len() != 2 {
        let _ = stream.write_all(b"INVALID REQUEST\n");
        return;
    }

    let (command, payload) = (parts[0], parts[1]);
    println!("命令: {}, 数据: {}", command, payload);

    match command {
        "PING" => {
            let _ = stream.write_all(b"PONG\n");
        }
        "ECHO" => {
            let _ = stream.write_all(payload.as_bytes());
        }
        _ => {
            let _ = stream.write_all(b"UNKNOWN COMMAND\n");
        }
    }
}

fn request_response_demo() {
    println!("=== 请求-响应服务器 ===");

    let listener = TcpListener::bind("127.0.0.1:8081").expect("绑定失败");
    println!("协议服务器监听在 127.0.0.1:8081");

    // 客户端线程
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));

        // PING 请求
        {
            let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
            stream.write_all(b"PING:hello").unwrap();
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            print!("客户端收到: {}", String::from_utf8_lossy(&buf[..n]));
        }

        // ECHO 请求
        {
            let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
            stream.write_all(b"ECHO:Hello World!\n").unwrap();
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            print!("客户端收到: {}", String::from_utf8_lossy(&buf[..n]));
        }
    });

    // 处理 2 个请求
    for _ in 0..2 {
        match listener.accept() {
            Ok((stream, _)) => handle_connection(stream),
            Err(_) => continue,
        }
    }

    thread::sleep(Duration::from_millis(100));
    println!("请求-响应演示完成");
    println!();
}

// ============================================================
// 示例 3: 多连接并发服务器
// ============================================================

fn concurrent_server_demo() {
    println!("=== 并发服务器 ===");

    let listener = TcpListener::bind("127.0.0.1:8082").expect("绑定失败");

    // 启动多个客户端线程
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));

        let mut handles = vec![];
        for i in 0..4 {
            let handle = thread::spawn(move || {
                let mut stream = TcpStream::connect("127.0.0.1:8082").unwrap();
                let msg = format!("客户端 {} 的消息\n", i);
                stream.write_all(msg.as_bytes()).unwrap();

                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf).unwrap();
                print!("客户端 {} 收到: {}", i, String::from_utf8_lossy(&buf[..n]));
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    });

    // 服务器用线程并发处理连接
    let mut handles = vec![];
    for _ in 0..4 {
        let (stream, _) = listener.accept().unwrap();
        let handle = thread::spawn(move || {
            let mut stream = stream;
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            stream.write_all(&buf[..n]).unwrap();
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("并发服务器演示完成");
    println!();
}

fn main() {
    echo_server_demo();
    request_response_demo();
    concurrent_server_demo();
}
