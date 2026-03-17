const std = @import("std");

// ============================================================
// build.zig - Zig 的构建脚本，用 Zig 代码编写！
//
// 这不是 JSON、YAML 或 TOML —— 这是完整的 Zig 代码，
// 你可以使用循环、条件判断、函数调用等所有语言特性。
// ============================================================

pub fn build(b: *std.Build) void {
    // 目标平台（默认为当前平台，可通过 -Dtarget= 覆盖）
    const target = b.standardTargetOptions(.{});

    // 优化模式（可通过 -Doptimize= 设置）
    const optimize = b.standardOptimizeOption(.{});

    // ============================================================
    // 1. 创建根模块（Zig 0.15+ 使用 Module 方式）
    // ============================================================
    const root_module = b.createModule(.{
        .root_source_file = b.path("main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // ============================================================
    // 2. 添加可执行文件
    // ============================================================
    const exe = b.addExecutable(.{
        .name = "build-demo",
        .root_module = root_module,
    });

    // 安装可执行文件到 zig-out/bin/
    b.installArtifact(exe);

    // ============================================================
    // 3. 添加运行步骤（zig build run）
    // ============================================================
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    // 允许传递命令行参数: zig build run -- arg1 arg2
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "运行程序");
    run_step.dependOn(&run_cmd.step);

    // ============================================================
    // 4. 添加测试步骤（zig build test）
    // ============================================================
    const test_module = b.createModule(.{
        .root_source_file = b.path("main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const unit_tests = b.addTest(.{
        .root_module = test_module,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "运行单元测试");
    test_step.dependOn(&run_unit_tests.step);

    // ============================================================
    // 5. 添加静态库（演示）
    // ============================================================
    const lib_module = b.createModule(.{
        .root_source_file = b.path("main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const lib = b.addLibrary(.{
        .linkage = .static,
        .name = "mylib",
        .root_module = lib_module,
    });

    b.installArtifact(lib);

    // ============================================================
    // 说明：更多构建系统能力
    // ============================================================
    //
    // 链接系统 C 库:
    //   root_module.link_libc = true;
    //   exe.linkSystemLibrary("sqlite3");
    //
    // 编译 C 源文件:
    //   root_module.addCSourceFile(.{
    //       .file = b.path("vendor/helper.c"),
    //       .flags = &.{"-Wall", "-O2"},
    //   });
    //
    // 添加头文件搜索路径:
    //   root_module.addIncludePath(b.path("include"));
    //
    // 自定义编译选项:
    //   const enable_log = b.option(bool, "enable-log", "启用日志") orelse false;
    //   const options = b.addOptions();
    //   options.addOption(bool, "enable_log", enable_log);
    //   root_module.addOptions("config", options);
    //
    // 交叉编译只需:
    //   zig build -Dtarget=aarch64-linux-gnu
    //   zig build -Dtarget=x86_64-windows-gnu -Doptimize=ReleaseFast
}
