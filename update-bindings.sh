#!/bin/bash

# 更新 KCP 绑定的脚本
# 用法: ./scripts/update-bindings.sh

set -e

echo "Updating KCP bindings..."

# 确保在项目根目录
cd "$(dirname "$0")/.."

# 更新子模块
echo "Updating KCP submodule..."
git submodule update --init --recursive

# 获取当前 KCP commit ID
KCP_COMMIT=$(cd kcp && git rev-parse HEAD)
echo "Current KCP commit: $KCP_COMMIT"

# 检查是否已经存在对应的绑定文件
BINDING_FILE="src/${KCP_COMMIT}_bindings.rs"
if [ -f "$BINDING_FILE" ]; then
    echo "Bindings already exist for this KCP version: $BINDING_FILE"
    echo "No update needed."
    exit 0
fi

# 生成新的绑定
echo "Generating new bindings..."
cargo build --features bindgen

# 检查生成是否成功
if [ -f "$BINDING_FILE" ]; then
    echo "✅ Successfully generated bindings: $BINDING_FILE"
    echo ""
    echo "Next steps:"
    echo "1. Review the generated bindings file"
    echo "2. Test the build: cargo test"
    echo "3. Commit the changes:"
    echo "   git add $BINDING_FILE"
    echo "   git commit -m \"Update KCP bindings for commit $KCP_COMMIT\""
else
    echo "❌ Failed to generate bindings"
    exit 1
fi 