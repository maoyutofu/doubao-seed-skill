# install

Install doubao-seed-skill binary for the current system platform.

Automatically detects the OS and architecture, downloads the pre-built binary from GitHub releases if available, or falls back to cloning and compiling from source.

## Usage

```
/install
```

## Supported Platforms

- Linux: x86_64, aarch64
- macOS: x86_64, aarch64
- Windows: x86_64

## How it works

1. Detects current system platform (OS and architecture)
2. Attempts to download pre-built binary from GitHub releases
3. If release not found, clones the repository and compiles from source
4. Installs the binary to `~/.local/bin/` (or `%APPDATA%\Local\bin\` on Windows)

## Environment Variables

- `INSTALL_DIR`: Custom installation directory (default: `~/.local/bin/`)
- `GITHUB_TOKEN`: Optional GitHub token for higher API rate limits

---

# Implementation

```bash
#!/bin/bash
set -e

# Detect platform
detect_platform() {
  local os=$(uname -s)
  local arch=$(uname -m)

  case "$os" in
    Linux)
      case "$arch" in
        x86_64) echo "linux-x86_64" ;;
        aarch64) echo "linux-aarch64" ;;
        *) echo "unsupported"; exit 1 ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        x86_64) echo "macos-x86_64" ;;
        arm64) echo "macos-aarch64" ;;
        *) echo "unsupported"; exit 1 ;;
      esac
      ;;
    MINGW*|MSYS*|CYGWIN*)
      case "$arch" in
        x86_64) echo "windows-x86_64" ;;
        *) echo "unsupported"; exit 1 ;;
      esac
      ;;
    *)
      echo "unsupported"
      exit 1
      ;;
  esac
}

# Get latest release info
get_latest_release() {
  local repo="maoyutofu/doubao-seed-skill"
  local api_url="https://api.github.com/repos/$repo/releases/latest"

  if [ -n "$GITHUB_TOKEN" ]; then
    curl -s -H "Authorization: token $GITHUB_TOKEN" "$api_url"
  else
    curl -s "$api_url"
  fi
}

# Download binary from release
download_binary() {
  local platform=$1
  local install_dir=${INSTALL_DIR:-~/.local/bin}
  local temp_dir=$(mktemp -d)

  mkdir -p "$install_dir"

  echo "Fetching latest release..."
  local release_data=$(get_latest_release)

  # Determine file extension and asset name
  local asset_name
  local extract_cmd

  case "$platform" in
    linux-*)
      asset_name="doubao-seed-skill-${platform}.tar.gz"
      extract_cmd="tar -xzf"
      ;;
    macos-*)
      asset_name="doubao-seed-skill-${platform}.tar.gz"
      extract_cmd="tar -xzf"
      ;;
    windows-*)
      asset_name="doubao-seed-skill-${platform}.zip"
      extract_cmd="unzip -q"
      ;;
  esac

  # Find download URL
  local download_url=$(echo "$release_data" | grep -o "\"browser_download_url\": \"[^\"]*$asset_name\"" | head -1 | cut -d'"' -f4)

  if [ -z "$download_url" ]; then
    echo "No release found for platform: $platform"
    return 1
  fi

  echo "Downloading from: $download_url"
  cd "$temp_dir"
  curl -L -o "release.${asset_name##*.}" "$download_url"

  echo "Extracting..."
  $extract_cmd "release.${asset_name##*.}"

  # Find and install binary
  local binary_name="doubao-seed-skill"
  if [ -f "$binary_name" ]; then
    chmod +x "$binary_name"
    mv "$binary_name" "$install_dir/"
    echo "✓ Installed to $install_dir/$binary_name"
  else
    echo "Binary not found in release"
    cd -
    rm -rf "$temp_dir"
    return 1
  fi

  cd -
  rm -rf "$temp_dir"
  return 0
}

# Build from source
build_from_source() {
  local install_dir=${INSTALL_DIR:-~/.local/bin}
  local temp_dir=$(mktemp -d)

  mkdir -p "$install_dir"

  echo "Cloning repository..."
  cd "$temp_dir"
  git clone https://github.com/maoyutofu/doubao-seed-skill.git
  cd doubao-seed-skill

  echo "Building from source..."
  cargo build --release

  echo "Installing..."
  cp target/release/doubao-seed-skill "$install_dir/"
  chmod +x "$install_dir/doubao-seed-skill"
  echo "✓ Installed to $install_dir/doubao-seed-skill"

  cd -
  rm -rf "$temp_dir"
}

# Main
main() {
  echo "Detecting platform..."
  local platform=$(detect_platform)
  echo "Platform: $platform"

  if download_binary "$platform"; then
    echo "✓ Installation complete"
    exit 0
  fi

  echo "Release not available, building from source..."
  build_from_source
  echo "✓ Installation complete"
}

main
```

## Notes

- The binary is installed to `~/.local/bin/` by default. Make sure this directory is in your `$PATH`.
- On Windows, the installation directory is `%APPDATA%\Local\bin\`.
- Requires `git` and `cargo` for source compilation fallback.
- Requires `curl` for downloading releases.
