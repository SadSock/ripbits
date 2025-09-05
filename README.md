# RipBits - 前后端分离版

一个用于二进制位操作和数值转换的工具，现已改造为前后端分离架构。

## 项目结构

```
ripbits/
├── backend/          # 后端API服务 (Axum)
├── frontend/         # 前端应用 (Dioxus WASM)
├── shared/           # 共享类型定义
└── README.md
```

## 功能特性

- 32位二进制可视化
- 多格式数值转换：十六进制、u32、i32、f32
- 实时双向同步
- REST API支持

## 快速启动

### 方式一：一键启动脚本

**Web版本 (浏览器)**
```bash
./run-web.sh
# 访问: http://localhost:8080
```

**Desktop版本 (原生桌面应用)**
```bash
./run-desktop.sh
# 自动打开桌面应用窗口
```

### 方式二：手动启动

**启动后端服务**
```bash
cd backend
cargo run
# 后端运行在 http://localhost:3000
```

**启动Web前端**
```bash
cd frontend
trunk serve  # 需要先安装: cargo install trunk
# Web版本运行在 http://localhost:8080
```

**启动Desktop前端**
```bash
cd frontend
cargo run
# 打开原生桌面应用窗口
```

## 支持平台

- ✅ **Web浏览器**: 使用WASM技术，支持所有现代浏览器
- ✅ **桌面应用**: 原生桌面窗口，支持 Windows、macOS、Linux
- 📱 **移动端**: 理论支持，需要额外配置

## API 接口

### GET /convert
查询参数：
- `value`: 要转换的值
- `format`: 值的格式 (`hex`, `uint32`, `int32`, `float32`)

### POST /convert
请求体：
```json
{
    "value": 123,
    "format": "uint32"
}
```

响应：
```json
{
    "hex": "0x0000007B",
    "uint32": "123", 
    "int32": "123",
    "float32": "1.724e-43",
    "bits": [false, false, ..., true]
}
```

## 技术栈

- **后端**: Rust + Axum + Tokio
- **前端**: Rust + Dioxus + WASM
- **共享**: Serde 序列化