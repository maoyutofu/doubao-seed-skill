# Skills

Custom Claude Code skills for doubao-seed-skill project.

## Available Skills

### `/install`

Automatically install doubao-seed-skill binary for your system.

**Description:**

Install doubao-seed-skill binary for the current system platform. Automatically detects the OS and architecture, downloads the pre-built binary from GitHub releases if available, or falls back to cloning and compiling from source.

**Usage:**

```
/install
```

**Supported Platforms:**

- Linux: x86_64, aarch64
- macOS: x86_64, aarch64
- Windows: x86_64

**How it works:**

1. Detects current system platform (OS and architecture)
2. Attempts to download pre-built binary from GitHub releases
3. If release not found, clones the repository and compiles from source
4. Installs the binary to `~/.local/bin/` (or `%APPDATA%\Local\bin\` on Windows)

**Environment Variables:**

- `INSTALL_DIR` - Custom installation directory (default: `~/.local/bin/`)

**Notes:**

- The binary is installed to `~/.local/bin/` by default. Make sure this directory is in your `$PATH`.
- On Windows, the installation directory is `%APPDATA%\Local\bin\`.
- Requires `git` and `cargo` for source compilation fallback.
- Requires `curl` for downloading releases.

**Usage:**

```sh
# Use environment variable parameters
doubao-seed-skill --image-url ./photo.jpg --prompt "描述这张图片"

# Using CLI flag & remote image URLs
doubao-seed-skill --api-key sk-xxx --image-url https://example.com/img.png

# Using CLI flag &  local images (automatically convert to base64)
doubao-seed-skill --api-key sk-xxx --image-url ./photo.jpg --prompt "描述这张图片"

# Using CLI flag & custom model & interface address
doubao-seed-skill --api-key sk-xxx --base-url https://custom.api/v1 --model my-model --image-url ./img.png
```
**Parameter:**

| CLI flag | Environment variables | Default |
|---|---|---|
| `--api-key` | `ARK_API_KEY` | Required |
| `--model` | `ARK_MODEL` | `doubao-seed-2-0-lite-260215` |
| `--base-url` | `ARK_BASE_URL` | `https://ark.cn-beijing.volces.com/api/v3` |
| `--image-url` | `IMAGE_URL` | Example Image URL |
| `--prompt` | `PROMPT` | `What did you see?？` |

Use environment variable parameters first.
