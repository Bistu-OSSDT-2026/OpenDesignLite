#!/bin/sh
# Open Design Lite installer (macOS / Linux).
# Usage: curl -fsSL https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.sh | sh
# Downloads the latest release binary to ~/.local/bin/odl.
# Spec: docs/specs/setup.md

set -eu

REPO="Bistu-OSSDT-2026/OpenDesignLite"
INSTALL_DIR="${HOME}/.local/bin"
TARGET="${INSTALL_DIR}/odl"

os="$(uname -s)"
arch="$(uname -m)"
case "$os" in
    Linux)
        case "$arch" in
            x86_64) asset="odl-linux-x64" ;;
            *) echo "error: unsupported Linux arch: $arch (release assets: x86_64 only)" >&2; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$arch" in
            arm64) asset="odl-macos-arm64" ;;
            x86_64) asset="odl-macos-x64" ;;
            *) echo "error: unsupported macOS arch: $arch" >&2; exit 1 ;;
        esac
        ;;
    *)
        echo "error: unsupported OS: $os (Windows users: run scripts/install.ps1)" >&2
        exit 1
        ;;
esac

url="https://github.com/${REPO}/releases/latest/download/${asset}"
echo "Downloading ${asset} from the latest release..."
mkdir -p "$INSTALL_DIR"
curl -fSL -o "$TARGET" "$url"
chmod +x "$TARGET"

echo "Installed: $TARGET"
"$TARGET" --version

# PATH guidance only - we do not edit shell rc files silently.
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo ""
        echo "NOTE: $INSTALL_DIR is not on your PATH."
        echo "Add it (bash/zsh) with:"
        echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc  # or ~/.zshrc"
        ;;
esac

echo ""
echo "Next step: wire up your coding agent with:"
echo "  odl setup"
