package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

// ============================================================
// Go HTTP 服务器 — 使用标准库 net/http
// ============================================================

// === 路由处理函数 ===

func handleIndex(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	fmt.Fprint(w, `<!DOCTYPE html>
<html>
<head><title>Go HTTP Server</title></head>
<body>
  <h1>Welcome to Go HTTP Server!</h1>
  <ul>
    <li><a href="/hello?name=World">/hello?name=World</a></li>
    <li><a href="/json">/json</a></li>
  </ul>
</body>
</html>`)
}

func handleHello(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	if name == "" {
		name = "World"
	}

	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	fmt.Fprintf(w, "Hello, %s!\n", name)
}

func handleJson(w http.ResponseWriter, r *http.Request) {
	data := map[string]any{
		"message":  "Hello from Go!",
		"version":  "1.22",
		"features": []string{"simple", "powerful", "productive"},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(data)
}

func handleEcho(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	io.Copy(w, r.Body)
}

// === 中间件示例 ===

func loggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s\n", r.Method, r.URL.Path)
		next.ServeHTTP(w, r)
	})
}

// ============================================================

func main() {
	mux := http.NewServeMux()

	// 注册路由
	mux.HandleFunc("/", handleIndex)
	mux.HandleFunc("/hello", handleHello)
	mux.HandleFunc("/json", handleJson)
	mux.HandleFunc("/echo", handleEcho)

	// 包装中间件
	handler := loggingMiddleware(mux)

	fmt.Println("Go HTTP 服务器运行在 http://127.0.0.1:8080/")
	fmt.Println("按 Ctrl+C 停止")
	fmt.Println()

	log.Fatal(http.ListenAndServe("127.0.0.1:8080", handler))
}
