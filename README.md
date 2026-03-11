# doubao-seed-skill

Rust 实现的豆包图像分析技能，通过 OpenAI 兼容接口调用豆包视觉模型。

## 安装

### 快速安装（推荐）

使用 Claude Code skill 自动安装：

```sh
/install
```

该命令会自动：
- 检测系统平台（Linux/macOS/Windows）
- 从 GitHub releases 下载预编译二进制
- 如果没有对应的 release，则克隆源码编译

详见 [SKILL.md](./skills/doubao-seed-skill/SKILL.md) 了解所有可用的 skills 和使用说明。

### 手动编译

```sh
cargo build --release
```

## 使用

```sh
# 使用远程图片 URL
./target/release/doubao-seed-skill --api-key sk-xxx --image-url https://example.com/img.png

# 使用本地图片（自动转 base64）
./target/release/doubao-seed-skill --api-key sk-xxx --image-url ./photo.jpg --prompt "描述这张图片"

# 自定义模型和接口地址
./target/release/doubao-seed-skill --api-key sk-xxx --base-url https://custom.api/v1 --model my-model --image-url ./img.png
```

## 参数

| CLI flag | 环境变量 | 默认值 |
|---|---|---|
| `--api-key` | `ARK_API_KEY` | 必填 |
| `--model` | `ARK_MODEL` | `doubao-seed-2-0-lite-260215` |
| `--base-url` | `ARK_BASE_URL` | `https://ark.cn-beijing.volces.com/api/v3` |
| `--image-url` | `IMAGE_URL` | 示例图片 URL |
| `--prompt` | `PROMPT` | `你看见了什么？` |

CLI 参数优先，环境变量兜底。
