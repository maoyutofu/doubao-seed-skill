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
- `GITHUB_TOKEN` - Optional GitHub token for higher API rate limits

**Notes:**

- The binary is installed to `~/.local/bin/` by default. Make sure this directory is in your `$PATH`.
- On Windows, the installation directory is `%APPDATA%\Local\bin\`.
- Requires `git` and `cargo` for source compilation fallback.
- Requires `curl` for downloading releases.

**Implementation:** See [skills/install.md](./skills/install.md) for detailed implementation.
