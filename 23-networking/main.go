package main

import (
	"bufio"
	"fmt"
	"net"
	"strings"
	"time"
)

// ============================================================
// 示例 1: TCP Echo 服务器 + 客户端（自包含演示）
// ============================================================

func echoServerDemo() {
	fmt.Println("=== TCP Echo 服务器 ===")

	listener, err := net.Listen("tcp", "127.0.0.1:8080")
	if err != nil {
		fmt.Println("监听失败:", err)
		return
	}
	defer listener.Close()
	fmt.Println("服务器监听在 127.0.0.1:8080")

	// 在 goroutine 中启动客户端
	go func() {
		time.Sleep(100 * time.Millisecond)

		conn, err := net.Dial("tcp", "127.0.0.1:8080")
		if err != nil {
			fmt.Println("连接失败:", err)
			return
		}
		defer conn.Close()

		fmt.Println("客户端已连接")

		// 发送数据
		message := "Hello, Go Networking!\n"
		fmt.Fprintf(conn, message)
		fmt.Printf("发送: %s", message)

		// 读取回显
		reader := bufio.NewReader(conn)
		reply, _ := reader.ReadString('\n')
		fmt.Printf("收到回显: %s\n", reply)
	}()

	// 服务器接受一个连接
	conn, err := listener.Accept()
	if err != nil {
		fmt.Println("接受连接失败:", err)
		return
	}
	defer conn.Close()

	fmt.Printf("客户端连接: %s\n", conn.RemoteAddr())

	// 回显数据
	buf := make([]byte, 1024)
	n, _ := conn.Read(buf)
	fmt.Printf("服务端收到: %s", string(buf[:n]))
	conn.Write(buf[:n])

	time.Sleep(100 * time.Millisecond)
	fmt.Println()
}

// ============================================================
// 示例 2: 请求-响应服务器（简单协议）
// ============================================================

func handleConnection(conn net.Conn) {
	defer conn.Close()

	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		return
	}

	data := string(buf[:n])
	parts := strings.SplitN(data, ":", 2)

	if len(parts) != 2 {
		fmt.Fprintf(conn, "INVALID REQUEST\n")
		return
	}

	command, payload := parts[0], parts[1]
	fmt.Printf("命令: %s, 数据: %s\n", command, payload)

	switch command {
	case "PING":
		fmt.Fprintf(conn, "PONG\n")
	case "ECHO":
		fmt.Fprintf(conn, "%s", payload)
	default:
		fmt.Fprintf(conn, "UNKNOWN COMMAND\n")
	}
}

func requestResponseDemo() {
	fmt.Println("=== 请求-响应服务器 ===")

	listener, err := net.Listen("tcp", "127.0.0.1:8081")
	if err != nil {
		fmt.Println("监听失败:", err)
		return
	}
	defer listener.Close()
	fmt.Println("协议服务器监听在 127.0.0.1:8081")

	// 客户端在 goroutine 中发送请求
	go func() {
		time.Sleep(100 * time.Millisecond)

		// PING 请求
		conn, _ := net.Dial("tcp", "127.0.0.1:8081")
		fmt.Fprintf(conn, "PING:hello")
		buf := make([]byte, 1024)
		n, _ := conn.Read(buf)
		fmt.Printf("客户端收到: %s", string(buf[:n]))
		conn.Close()

		// ECHO 请求
		conn, _ = net.Dial("tcp", "127.0.0.1:8081")
		fmt.Fprintf(conn, "ECHO:Hello World!\n")
		n, _ = conn.Read(buf)
		fmt.Printf("客户端收到: %s", string(buf[:n]))
		conn.Close()
	}()

	// 处理 2 个请求
	for i := 0; i < 2; i++ {
		conn, err := listener.Accept()
		if err != nil {
			continue
		}
		handleConnection(conn)
	}

	time.Sleep(100 * time.Millisecond)
	fmt.Println("请求-响应演示完成")
	fmt.Println()
}

// ============================================================
// 示例 3: 多连接并发服务器
// ============================================================

func concurrentServerDemo() {
	fmt.Println("=== 并发服务器 ===")

	listener, err := net.Listen("tcp", "127.0.0.1:8082")
	if err != nil {
		fmt.Println("监听失败:", err)
		return
	}
	defer listener.Close()

	done := make(chan bool)

	// 启动多个并发客户端
	go func() {
		time.Sleep(100 * time.Millisecond)
		for i := 0; i < 4; i++ {
			go func(id int) {
				conn, _ := net.Dial("tcp", "127.0.0.1:8082")
				defer conn.Close()
				fmt.Fprintf(conn, "客户端 %d 的消息\n", id)
				buf := make([]byte, 1024)
				n, _ := conn.Read(buf)
				fmt.Printf("客户端 %d 收到: %s", id, string(buf[:n]))
			}(i)
		}
	}()

	// 服务器并发处理 4 个连接
	for i := 0; i < 4; i++ {
		conn, _ := listener.Accept()
		go func(c net.Conn) {
			defer c.Close()
			buf := make([]byte, 1024)
			n, _ := c.Read(buf)
			c.Write(buf[:n]) // 回显
		}(conn)
	}

	go func() {
		time.Sleep(500 * time.Millisecond)
		done <- true
	}()

	<-done
	fmt.Println("并发服务器演示完成")
	fmt.Println()
}

func main() {
	echoServerDemo()
	requestResponseDemo()
	concurrentServerDemo()
}
