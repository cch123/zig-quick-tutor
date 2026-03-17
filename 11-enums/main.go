package main

import "fmt"

// === Go 没有真正的枚举，使用 const + iota 模拟 ===

type Direction int

const (
	North Direction = iota
	South
	East
	West
)

// 为"枚举"实现 String() 方法
func (d Direction) String() string {
	switch d {
	case North:
		return "North"
	case South:
		return "South"
	case East:
		return "East"
	case West:
		return "West"
	default:
		return "Unknown"
	}
}

// === 带自定义值的"枚举" ===
type HttpStatus int

const (
	StatusOK            HttpStatus = 200
	StatusNotFound      HttpStatus = 404
	StatusInternalError HttpStatus = 500
)

func (s HttpStatus) String() string {
	switch s {
	case StatusOK:
		return "OK"
	case StatusNotFound:
		return "Not Found"
	case StatusInternalError:
		return "Internal Server Error"
	default:
		return "Unknown"
	}
}

// === 带方法的"枚举" ===
type Season int

const (
	Spring Season = iota
	Summer
	Autumn
	Winter
)

func (s Season) IsWarm() bool {
	return s == Spring || s == Summer
}

func (s Season) Next() Season {
	return (s + 1) % 4
}

func (s Season) Name() string {
	names := [...]string{"Spring", "Summer", "Autumn", "Winter"}
	if int(s) < len(names) {
		return names[s]
	}
	return "Unknown"
}

// === 用字符串枚举 ===
type Color string

const (
	Red   Color = "red"
	Green Color = "green"
	Blue  Color = "blue"
)

func main() {
	// === 基本使用 ===
	dir := North
	fmt.Println("方向:", dir)

	// === switch（不强制穷尽） ===
	switch dir {
	case North:
		fmt.Println("方向描述: 向北")
	case South:
		fmt.Println("方向描述: 向南")
	case East:
		fmt.Println("方向描述: 向东")
	case West:
		fmt.Println("方向描述: 向西")
	}

	// === 枚举方法 ===
	season := Summer
	fmt.Println("季节:", season.Name())
	fmt.Println("温暖:", season.IsWarm())
	fmt.Println("下一个:", season.Next().Name())

	// === 类型安全的弱点 ===
	// 任何 int 都能赋值给 Direction（类型别名）
	var invalid Direction = 99 // 编译通过！
	fmt.Println("无效方向:", invalid)

	// === HTTP 状态码 ===
	status := StatusNotFound
	fmt.Printf("HTTP 状态码: %d (%s)\n", int(status), status)

	// === 字符串枚举 ===
	color := Red
	fmt.Println("颜色:", color)

	// === iota 高级用法 ===
	type FilePermission int
	const (
		Read    FilePermission = 1 << iota // 1
		Write                              // 2
		Execute                            // 4
	)
	perms := Read | Write
	fmt.Printf("权限: %d (读=%v, 写=%v, 执行=%v)\n",
		perms,
		perms&Read != 0,
		perms&Write != 0,
		perms&Execute != 0,
	)

	// === 季节温度映射 ===
	temps := map[Season]int{
		Spring: 20,
		Summer: 35,
		Autumn: 15,
		Winter: -5,
	}
	fmt.Println("\n季节温度:")
	for s := Spring; s <= Winter; s++ {
		fmt.Printf("  %s: %d°C\n", s.Name(), temps[s])
	}
}
