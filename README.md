# Open Design Lite

> **Local-first design assistant (`odl`) that plugs into your coding agent over MCP.**
>
> Describe what you want in one sentence — the agent calls Open Design Lite's tools to create HTML pages, Markdown docs, or lightweight HTML slides, and a live preview window pops open and refreshes as the agent iterates.

Design defaults stay lightweight and framework-agnostic: **CSS variables, static recipes, and plain files** instead of a bundled UI runtime. No React, no Tailwind, no shadcn — just portable, inspectable artifacts on disk.

**中文文档**: [docs/README.md](docs/README.md)

---

## Table of Contents

- [Features](#features)
- [How It Works](#how-it-works)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [CLI Mode (Scripting)](#cli-mode-scripting)
  - [MCP Mode (Agent-Driven)](#mcp-mode-agent-driven)
  - [Agent Configuration](#agent-configuration)
- [MCP Tools Reference](#mcp-tools-reference)
- [Built-in Skills](#built-in-skills)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- 🔧 **MCP-First** — Agents drive design via MCP tools; the CLI is a convenience over the same kernel.
- 🖥️ **Local-First** — No cloud, no server, no sign-up. Everything runs on your machine.
- 🪟 **Live Preview** — A native WebView window auto-opens and live-reloads on every change.
- 🎨 **Framework-Agnostic** — Design tokens (`--od-*` CSS variables) and static `.od-*` class recipes instead of a runtime.
- 📄 **Multiple Artifact Types** — HTML pages, Markdown docs, and lightweight HTML slides.
- 📦 **Export Ready** — Export to self-contained HTML, Markdown, ZIP, or PDF.
- 🤝 **Handoff-Friendly** — Every artifact includes a `handoff.md` so humans and agents stay in sync.
- 🧩 **Extensible Skills** — Built-in skills are filesystem directories with readable `SKILL.md` files.

---

## How It Works

```
You (one sentence) → Coding Agent → MCP Tools → od-core → Artifact on disk → Live Preview
```

1. **Configure** the `open-design-lite` MCP server once in your coding agent.
2. **Describe** what you want in natural language.
3. **Agent calls** MCP tools (`artifact_create`, `artifact_preview`, …) to build the artifact.
4. **Preview auto-opens** in a native window and live-reloads as the agent iterates.
5. **Keep chatting** to refine; export when done.

No separate app, no chat/preview shell to install — **the agent is your UI, the preview is just a window**.

---

## Prerequisites

- **[Rust](https://rustup.rs/)** toolchain (stable, edition 2021)
- **Windows** (primary target; macOS/Linux support tracked on roadmap)
- A compatible coding agent (see [Agent Configuration](#agent-configuration))

---

## Installation

### Build from Source

```powershell
# Clone the repository
git clone https://github.com/Bistu-OSSDT-2026/OpenDesignLite.git
cd OpenDesignLite

# Build all crates
cargo build --release

# (Windows) If link.exe issues occur, use the helper script:
powershell -File scripts/build.ps1 build --release
```

The release binary will be at `target/release/odl.exe`.

### Verify Installation

```powershell
cargo run -p od-cli -- --help
```

---

## Usage

### CLI Mode (Scripting)

The `odl` CLI lets you create and preview artifacts from the terminal:

```powershell
# Initialize a workspace
cargo run -p od-cli -- init my-workspace

# Create a new HTML artifact
cargo run -p od-cli -- new html my-workspace/hello

# Create a Markdown document
cargo run -p od-cli -- new docs my-workspace/readme

# Create HTML slides
cargo run -p od-cli -- new slides my-workspace/deck

# Preview an artifact (opens native live-reload window)
cargo run -p od-cli -- preview my-workspace/hello

# Export an artifact
cargo run -p od-cli -- export html my-workspace/hello   # self-contained HTML
cargo run -p od-cli -- export md my-workspace/readme    # Markdown
cargo run -p od-cli -- export zip my-workspace          # ZIP archive
cargo run -p od-cli -- export pdf my-workspace/hello    # PDF (requires Chrome/Edge)
```

### MCP Mode (Agent-Driven)

Start the MCP server for agent integration:

```powershell
cargo run -p od-mcp -- mcp
# Or with the release binary:
odl mcp
```

This starts a JSON-RPC server over stdio that your coding agent connects to. See [Agent Configuration](#agent-configuration) below for setup instructions per agent.

### Agent Configuration

Configure the MCP server in your agent's settings. Choose your agent:

<details>
<summary><strong>Claude Code</strong></summary>

Add to `~/.claude/claude_desktop_config.json` or project `.mcp.json`:

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "cargo",
      "args": ["run", "-p", "od-mcp", "--", "mcp"],
      "cwd": "/path/to/OpenDesignLite"
    }
  }
}
```
</details>

<details>
<summary><strong>Codex (OpenCode)</strong></summary>

Add to `opencode.json`:

```json
{
  "mcp": {
    "open-design-lite": {
      "command": "cargo",
      "args": ["run", "-p", "od-mcp", "--", "mcp"],
      "workdir": "/path/to/OpenDesignLite"
    }
  }
}
```
</details>

<details>
<summary><strong>Cursor</strong></summary>

Add to Cursor's MCP config (`~/.cursor/mcp.json`):

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "cargo",
      "args": ["run", "-p", "od-mcp", "--", "mcp"],
      "cwd": "/path/to/OpenDesignLite"
    }
  }
}
```
</details>

<details>
<summary><strong>Zed</strong></summary>

Add to Zed's `settings.json`:

```json
{
  "context_servers": {
    "open-design-lite": {
      "command": {
        "path": "cargo",
        "args": ["run", "-p", "od-mcp", "--", "mcp"],
        "work_dir": "/path/to/OpenDesignLite"
      }
    }
  }
}
```
</details>

<details>
<summary><strong>ZCode / Other MCP-compatible agents</strong></summary>

Configure as a stdio MCP server with:

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "cargo",
      "args": ["run", "-p", "od-mcp", "--", "mcp"],
      "cwd": "/path/to/OpenDesignLite"
    }
  }
}
```

After configuring, restart your agent. You should see `artifact_create`, `artifact_preview`, `artifact_handoff`, and `artifact_export` tools available.
</details>

---

## MCP Tools Reference

| Tool | Description |
|------|-------------|
| `artifact_create` | Create a new artifact (HTML page, Markdown doc, or slides) in a workspace. |
| `artifact_preview` | Open or refresh the live-reloading preview window for an artifact directory. |
| `artifact_handoff` | Generate or read the `handoff.md` for an artifact — keeps agents and humans in sync. |
| `artifact_export` | Export an artifact to a portable format: `html`, `md`, `zip`, or `pdf`. |

**Preview defaults**: `externalBrowser: false`, `watch: true` — uses the MCP-managed `odl` preview window with live-reload. Do not open a system browser yourself; the preview skill handles it (see `skills/preview-via-mcp/SKILL.md`).

---

## Built-in Skills

Skills are filesystem directories with readable `SKILL.md` files that agents can discover and use:

| Skill | Mode | Description |
|-------|------|-------------|
| `html-page` | `html` | Create a self-contained HTML page with inline CSS (landing pages, articles, microsites). |
| `slides-html` | `slides` | Create lightweight HTML presentation slides (16:9, single-file). |
| `docs-polish` | `docs` | Polish and format Markdown documentation. |
| `preview-via-mcp` | `workflow` | Instructs agents to preview via MCP (never open system browser). |

**Visual briefs** (`skills/visual-briefs.md`) define three shared aesthetics referenced by skills:
- **Editorial** — Warm paper-like magazine feel (docs, landing pages, narrative).
- **Studio** — Gallery wall / design critique board (portfolios, showcases, slides).
- **Workbench** — Clear information hierarchy (tools, dashboards, workspaces).

---

## Project Structure

```text
OpenDesignLite/
├── crates/
│   ├── od-core/        # Kernel: artifacts, design tokens, workspaces, skills
│   ├── od-cli/         # CLI: init, new, preview, export, handoff, skill
│   ├── od-mcp/         # MCP server: JSON-RPC over stdio (agent bridge)
│   └── od-preview/     # Native WebView preview window with live-reload
├── docs/               # Documentation (中文): architecture, specs, ADRs, roadmap
├── skills/             # Built-in agent skills (SKILL.md per directory)
├── templates/          # Starter templates for each artifact type
├── scripts/            # Build helpers (Windows MSVC, MCP proxy)
├── Cargo.toml          # Rust workspace manifest
└── AGENTS.md           # Instructions for coding agents working on this repo
```

---

## Documentation

All detailed documentation is in the [`docs/`](docs/) directory (primarily in Chinese):

| Document | Purpose |
|----------|---------|
| [docs/README.md](docs/README.md) | Documentation index and reading guide |
| [docs/product/prd.md](docs/product/prd.md) | Product scope, exclusions, MVP definition |
| [docs/product/roadmap.md](docs/product/roadmap.md) | Milestones M0–M3 and release plan |
| [docs/architecture/overview.md](docs/architecture/overview.md) | System layers, module boundaries, data model |
| [docs/decisions/](docs/decisions/) | Architecture Decision Records (ADRs) |
| [docs/specs/](docs/specs/) | Per-module implementation specs and interface contracts |
| [docs/releases/](docs/releases/) | Release notes and known limitations |
| [docs/team/plan.md](docs/team/plan.md) | Team structure and work division |

---

## Roadmap

| Milestone | Status | Focus |
|-----------|--------|-------|
| **M0** – Repository Contract | ✅ Done | Docs, scaffold, minimal CLI, skill directories, starter templates |
| **M1** – Local Artifact Loop | ✅ Done | `init`, `new html\|docs\|slides`, native preview with auto-refresh, handoff |
| **M2** – Agent Bridging (MCP) | 🚧 Finishing | MCP server, agent config docs, end-to-end agent→preview loop |
| **M3** – Export | 🚧 Finishing | HTML/MD/ZIP/PDF export, single-file HTML (planned) |

See [docs/product/roadmap.md](docs/product/roadmap.md) for details.

---

## Contributing

Contributions are welcome! Please read the agent instructions in [AGENTS.md](AGENTS.md) for the engineering rules and product principles that guide this project.

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to your branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

Keep changes small, fast, and boring unless the product goal requires otherwise. Do not add heavy frameworks without an ADR.

---

## License

This project is licensed under the **MIT License** — see [LICENSE](LICENSE) for details.
