use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// ============================================================
// Rust HTTP 服务器 — 不使用第三方库，手动解析 HTTP
// 展示 HTTP 协议的底层原理
// ============================================================

// === HTTP 请求解析 ===

struct HttpRequest {
    method: String,
    path: String,
    query: String,
    body: String,
    content_length: usize,
}

fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);

    // 读取请求行
    let mut request_line = String::new();
    reader.read_line(&mut request_line).ok()?;

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let method = parts[0].to_string();
    let full_path = parts[1].to_string();

    // 分离 path 和 query string
    let (path, query) = match full_path.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (full_path, String::new()),
    };

    // 读取 headers
    let mut content_length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        if line.trim().is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            content_length = line.split(':').nth(1)?.trim().parse().unwrap_or(0);
        }
    }

    // 读取 body
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        use std::io::Read;
        reader.read_exact(&mut body).ok()?;
    }

    Some(HttpRequest {
        method,
        path,
        query,
        body: String::from_utf8_lossy(&body).to_string(),
        content_length,
    })
}

// === 路由处理函数 ===

fn serve_index(stream: &mut TcpStream) {
    let body = r#"<!DOCTYPE html>
<html>
<head><title>Rust HTTP Server</title></head>
<body>
  <h1>Welcome to Rust HTTP Server!</h1>
  <ul>
    <li><a href="/hello?name=World">/hello?name=World</a></li>
    <li><a href="/json">/json</a></li>
  </ul>
</body>
</html>"#;

    send_response(stream, 200, "text/html; charset=utf-8", body);
}

fn serve_hello(stream: &mut TcpStream, query: &str) {
    // 解析 name 参数
    let name = query
        .split('&')
        .find_map(|param| {
            let (key, value) = param.split_once('=')?;
            if key == "name" {
                Some(value)
            } else {
                None
            }
        })
        .unwrap_or("World");

    let body = format!("Hello, {}!\n", name);
    send_response(stream, 200, "text/plain; charset=utf-8", &body);
}

fn serve_json(stream: &mut TcpStream) {
    // 手动构建 JSON（生产环境应使用 serde_json）
    let body = r#"{"message": "Hello from Rust!", "version": "1.77", "features": ["fast", "safe", "concurrent"]}"#;
    send_response(stream, 200, "application/json", body);
}

fn serve_echo(stream: &mut TcpStream, body: &str) {
    send_response(stream, 200, "text/plain", body);
}

fn serve_not_found(stream: &mut TcpStream) {
    send_response(stream, 404, "text/plain", "404 Not Found\n");
}

fn serve_method_not_allowed(stream: &mut TcpStream) {
    send_response(stream, 405, "text/plain", "405 Method Not Allowed\n");
}

// === HTTP 响应构建 ===

fn send_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Unknown",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        status_text,
        content_type,
        body.len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
}

// ============================================================

fn handle_connection(mut stream: TcpStream) {
    let request = match parse_request(&mut stream) {
        Some(req) => req,
        None => return,
    };

    println!("{} {}", request.method, request.path);

    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => serve_index(&mut stream),
        ("GET", "/hello") => serve_hello(&mut stream, &request.query),
        ("GET", "/json") => serve_json(&mut stream),
        ("POST", "/echo") => serve_echo(&mut stream, &request.body),
        (_, "/echo") => serve_method_not_allowed(&mut stream),
        _ => serve_not_found(&mut stream),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("绑定失败");

    println!("Rust HTTP 服务器运行在 http://127.0.0.1:8080/");
    println!("按 Ctrl+C 停止");
    println!();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 每个连接一个线程
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("连接失败: {}", e);
            }
        }
    }
}
