package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	// === 字符串基础 ===
	hello := "Hello, Go!"
	fmt.Println("字符串:", hello)
	fmt.Println("长度(字节):", len(hello))

	// 访问单个字节
	fmt.Printf("第一个字节: '%c' (0x%x)\n", hello[0], hello[0])

	// === 字符串拼接 ===
	// 简单拼接（每次产生新分配）
	greeting := "Hello" + ", " + "World!"
	fmt.Println("简单拼接:", greeting)

	// 高效拼接：strings.Builder
	var builder strings.Builder
	builder.WriteString("Hello")
	builder.WriteString(", ")
	builder.WriteString("World!")
	fmt.Println("Builder 拼接:", builder.String())

	// strings.Join
	parts := []string{"Hello", ", ", "World!"}
	joined := strings.Join(parts, "")
	fmt.Println("Join 拼接:", joined)

	// === 字符串格式化 ===
	formatted := fmt.Sprintf("name=%s, age=%d", "Alice", 30)
	fmt.Println("格式化:", formatted)

	// === 多行字符串 ===
	multiline := `这是第一行
这是第二行
这是第三行`
	fmt.Println("多行字符串:")
	fmt.Println(multiline)

	// === 字符串比较 ===
	a := "hello"
	b := "hello"
	c := "world"
	fmt.Println("a == b:", a == b)
	fmt.Println("a == c:", a == c)

	// === 字符串查找 ===
	haystack := "Hello, World!"
	index := strings.Index(haystack, "World")
	fmt.Printf("找到 'World' 在位置: %d\n", index)

	// === 遍历字节 ===
	fmt.Print("逐字节: ")
	for i := 0; i < len("Go!"); i++ {
		fmt.Printf("%c ", "Go!"[i])
	}
	fmt.Println()

	// === 遍历 rune（Unicode 码点） ===
	chinese := "你好世界"
	fmt.Println("UTF-8 字节长度:", len(chinese))
	fmt.Println("UTF-8 字符数:", utf8.RuneCountInString(chinese))

	fmt.Print("逐字符: ")
	for _, r := range chinese {
		fmt.Printf("%c ", r)
	}
	fmt.Println()

	// === rune 和 byte 的区别 ===
	s := "Hello, 世界"
	fmt.Printf("字节切片: %v\n", []byte(s))
	fmt.Printf("rune 切片: %v\n", []rune(s))

	// === 字符串不可变 ===
	// s[0] = 'h'  // 编译错误：字符串不可变
	// 需要转换为 []byte 修改
	bs := []byte(s)
	bs[0] = 'h'
	modified := string(bs)
	fmt.Println("修改后:", modified)
}
