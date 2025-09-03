# Dioxus Checkbox Counter

这是一个使用 Dioxus 框架创建的 Rust 应用程序，包含32个复选框并统计选中的数量。

## 功能特性

- 32个复选框，排列成4x8的网格布局
- 实时统计选中的复选框数量
- 支持 Web 和桌面端运行

## 运行方式

### 桌面端
```bash
cargo run
```

### Web端
1. 安装 dx CLI工具：
```bash
cargo install dioxus-cli
```

2. 运行 Web 服务器：
```bash
dx serve --platform web
```

或者使用以下命令构建 Web 版本：
```bash
dx build --platform web
```

然后在浏览器中打开 `index.html` 文件。

## 项目结构

- `src/main.rs` - 主应用程序代码
- `Cargo.toml` - 项目依赖配置
- `index.html` - Web 版本的 HTML 入口文件

## 依赖项

- `dioxus`: 支持 Web 和桌面的 Rust GUI 框架