# doubao-seed-skill

豆包图像分析技能：调用豆包（字节跳动）视觉大模型，分析图片内容。

## 安装

从 GitHub Release 下载对应平台的二进制文件：

**Release 地址：** `https://github.com/maoyutofu/doubao-seed-skill/releases/latest`

### 自动检测平台并下载

根据当前系统自动选择正确的包：

| 系统 | 架构 | 文件名 |
|------|------|--------|
| Linux | x86_64 | `doubao-seed-skill-{version}-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | aarch64 | `doubao-seed-skill-{version}-aarch64-unknown-linux-gnu.tar.gz` |
| macOS | x86_64 (Intel) | `doubao-seed-skill-{version}-x86_64-apple-darwin.tar.gz` |
| macOS | aarch64 (Apple Silicon) | `doubao-seed-skill-{version}-aarch64-apple-darwin.tar.gz` |
| Windows | x86_64 | `doubao-seed-skill-{version}-x86_64-pc-windows-msvc.zip` |

### 下载步骤（Linux / macOS）

```bash
# 1. 获取最新版本号
VERSION=$(curl -s https://api.github.com/repos/maoyutofu/doubao-seed-skill/releases/latest | grep '"tag_name"' | sed 's/.*"tag_name": "\(.*\)".*/\1/')

# 2. 检测系统和架构
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# 3. 映射到 target triple
if [ "$OS" = "linux" ] && [ "$ARCH" = "x86_64" ]; then
  TARGET="x86_64-unknown-linux-gnu"
elif [ "$OS" = "linux" ] && [ "$ARCH" = "aarch64" ]; then
  TARGET="aarch64-unknown-linux-gnu"
elif [ "$OS" = "darwin" ] && [ "$ARCH" = "x86_64" ]; then
  TARGET="x86_64-apple-darwin"
elif [ "$OS" = "darwin" ] && [ "$ARCH" = "arm64" ]; then
  TARGET="aarch64-apple-darwin"
fi

# 4. 下载并解压
ARCHIVE="doubao-seed-skill-${VERSION}-${TARGET}.tar.gz"
curl -LO "https://github.com/maoyutofu/doubao-seed-skill/releases/download/${VERSION}/${ARCHIVE}"
tar -xzf "$ARCHIVE"

# 5. 移动到 PATH（可选）
sudo mv doubao-seed-skill /usr/local/bin/
```

### 下载步骤（Windows PowerShell）

```powershell
# 1. 获取最新版本号
$VERSION = (Invoke-RestMethod "https://api.github.com/repos/maoyutofu/doubao-seed-skill/releases/latest").tag_name

# 2. 下载
$ARCHIVE = "doubao-seed-skill-${VERSION}-x86_64-pc-windows-msvc.zip"
Invoke-WebRequest "https://github.com/maoyutofu/doubao-seed-skill/releases/download/${VERSION}/${ARCHIVE}" -OutFile $ARCHIVE

# 3. 解压
Expand-Archive $ARCHIVE -DestinationPath .
```

## 配置

需要豆包 API Key，通过 CLI 参数或环境变量传入：

| CLI 参数 | 环境变量 | 默认值 |
|----------|----------|--------|
| `--api-key` | `ARK_API_KEY` | 必填 |
| `--model` | `ARK_MODEL` | `ep-20260306183709-wgl6s` |
| `--base-url` | `ARK_BASE_URL` | `https://ark.cn-beijing.volces.com/api/v3` |
| `--image-url` | `IMAGE_URL` | 示例图片 URL |
| `--prompt` | `PROMPT` | `你看见了什么？` |

推荐通过环境变量设置 API Key：

```bash
export ARK_API_KEY="your-api-key-here"
```

## 使用

```bash
# 分析网络图片
doubao-seed-skill --api-key YOUR_KEY --image-url https://example.com/image.jpg

# 分析本地图片（自动转 base64）
doubao-seed-skill --api-key YOUR_KEY --image-url /path/to/local/image.png

# 自定义提示词
doubao-seed-skill --api-key YOUR_KEY --image-url /path/to/image.jpg --prompt "描述图中的主要物体"

# 使用环境变量（推荐）
export ARK_API_KEY="your-key"
doubao-seed-skill --image-url /path/to/image.jpg --prompt "这张图片里有什么？"
```

## 示例输出

```
图中是一只橙色的猫咪，正趴在窗台上晒太阳，背景是模糊的绿色植物。
```
