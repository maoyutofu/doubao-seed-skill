# Skills

Custom Claude Code skills for doubao-seed-skill project.

## Available Skills

### `/install`

Automatically install doubao-seed-skill binary for your system.

**Features:**
- Auto-detects OS and architecture (Linux, macOS, Windows)
- Downloads pre-built binary from GitHub releases
- Falls back to cloning and compiling from source if release unavailable
- Installs to `~/.local/bin/` (customizable via `INSTALL_DIR`)

**Usage:**
```
/install
```

**Supported Platforms:**
- Linux: x86_64, aarch64
- macOS: x86_64, aarch64
- Windows: x86_64

**Environment Variables:**
- `INSTALL_DIR` - Custom installation directory (default: `~/.local/bin/`)
- `GITHUB_TOKEN` - Optional GitHub token for higher API rate limits

See [install.md](./install.md) for implementation details.
