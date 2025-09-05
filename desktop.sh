#!/bin/bash

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🖥️  启动桌面版本"
echo "后端: http://localhost:3000"
echo ""

# 启动后端
echo "启动后端服务..."
cd "$SCRIPT_DIR/backend" && cargo run &
BACKEND_PID=$!

# 等待后端启动
sleep 3

# 启动桌面前端
echo "启动桌面应用..."
cd "$SCRIPT_DIR/frontend" && cargo run &
DESKTOP_PID=$!

echo ""
echo "✅ 服务已启动"
echo "🖥️  桌面应用窗口应该已经打开"
echo ""
echo "按 Ctrl+C 停止服务"

# 等待用户中断
trap "echo ''; echo '🛑 停止服务...'; kill $BACKEND_PID $DESKTOP_PID 2>/dev/null; exit 0" INT

wait