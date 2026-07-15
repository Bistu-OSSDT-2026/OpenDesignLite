# Agent Setup Guide

> **Instructions for coding agents to automatically install, build, and configure Open Design Lite.**
>
> Read this file when a user asks you to set up Open Design Lite, create a design artifact, or preview HTML/Markdown/Slides via MCP.

---

## What This Project Is

Open Design Lite (`odl`) is a **local-first design assistant** that you (the coding agent) drive through MCP tools. You call `artifact_create` to make HTML pages / Markdown docs / slides, `artifact_preview` to open a live-reloading preview window, and `artifact_export` to package the result. There is no web server, no cloud, no sign-up — everything is local.

---

## Quick Auto-Install

Two steps — no Rust toolchain, no clone, no manual JSON editing:

### Step 1: Install the Binary

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.ps1 | iex
```

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.sh | sh
```

The binary lands at `%LOCALAPPDATA%\OpenDesignLite\bin\odl.exe` (Windows) or `~/.local/bin/odl` (Unix). If the install directory is not on PATH, the script prints instructions — use the absolute path in the meantime.

### Step 2: Configure MCP

```bash
odl setup
```

This detects installed agents (Claude Code, Cursor, OpenCode, Zed, ZCode) and writes the `open-design-lite` MCP server entry into their config, pointing at the installed binary. Useful flags: `--agent <name>` (single agent), `--dry-run` (print without writing), `--global` (user-level config), `--force` (overwrite a stale entry, e.g. an old `cargo run` template). Full contract: [docs/specs/setup.md](docs/specs/setup.md).

After setup, **restart the agent** for MCP tools to appear.

**Manual fallback** (troubleshooting only — this is what `odl setup` generates; `<ODL>` = absolute binary path):

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "<ODL>",
      "args": ["mcp"]
    }
  }
}
```

**Agent-specific locations:**

| Agent | Config File |
|-------|-------------|
| Claude Code | project `.mcp.json` or user-level `~/.claude.json` |
| Codex / OpenCode | `opencode.json` → `mcp` section |
| Cursor | `.cursor/mcp.json` (project) or `~/.cursor/mcp.json` |
| Zed | `settings.json` → `context_servers` section (nested `command: {path, args}`) |
| ZCode | `.zcode/mcp.json` or workspace MCP settings |

> **Do NOT configure `cargo run` as the MCP command.** It recompiles on first call and build output on stdio can corrupt the JSON-RPC stream.

> Note: the `.mcp.json` + `CLAUDE.md` inside the OpenDesignLite repo itself only serve the dogfood case (working on this repo). For everyday use in the user's own projects, always go through `odl setup`.

### Contributor Path (Build from Source)

Only needed when working on OpenDesignLite itself: install Rust (https://rustup.rs/), `git clone`, `cargo build --release` (Windows link.exe issues → `powershell -File scripts/build.ps1 build --release`). The binary is `target/release/odl`.

---

## MCP Tools You Can Call

Once configured, these tools are available to you via MCP:

### `artifact_create`
Create a new artifact in a workspace.

**Parameters:**
- `mode` (string, required): `"html"`, `"docs"`, or `"slides"`
- `name` (string, required): artifact directory name (e.g., `"my-landing-page"`)
- `workspace` (string, optional): workspace directory path
- `visualBrief` (string, optional): `"editorial"`, `"studio"`, or `"workbench"` (default: `"editorial"`)

**What it does:** Creates a directory with starter files (`index.html`, `assets/od-design.css`, `manifest.json`, `handoff.md`) based on the chosen mode and visual brief. By default it also auto-opens the preview window (`autoPreview: true`); pass `autoPreview: false` for CI/scripted runs. The response includes a `nextStep` hint.

**Example call:**
```json
{
  "mode": "html",
  "name": "landing-page",
  "workspace": "/home/user/odl-workspace",
  "visualBrief": "editorial"
}
```

### `artifact_preview`
Open or refresh the live-reloading preview window.

**Parameters:**
- `dir` (string, required): absolute path to the artifact directory
- `externalBrowser` (boolean, optional): use system browser instead of native window (default: `false`)
- `watch` (boolean, optional): enable live-reload on file changes (default: `true`)

**CRITICAL RULES:**
- **Always** use this tool to preview — NEVER open a system browser yourself (`start`, `xdg-open`, `open`, Playwright, etc.).
- Keep defaults (`externalBrowser: false`, `watch: true`) unless the user explicitly asks otherwise.
- After the first preview, file edits trigger automatic live-reload; only call again if the window was closed.
- Calling it again for the same dir is safe: it returns `alreadyRunning: true` instead of opening a duplicate window.
- The window is fixed-size per artifact type (slides: 1280×720 16:9; html/docs: 1366×768) and not resizable — content renders at the exact deployed-viewport size.

**Example call:**
```json
{
  "dir": "/home/user/odl-workspace/landing-page"
}
```

### `artifact_handoff`
Generate or read a `handoff.md` file for an artifact. This keeps context between you and the user (or between different agents).

**Parameters:**
- `dir` (string, required): absolute path to the artifact directory
- `action` (string, optional): `"generate"` to create, `"read"` to read existing (default: `"generate"`)

### `artifact_export`
Export an artifact to a portable format.

**Parameters:**
- `dir` (string, required): absolute path to the artifact directory
- `format` (string, required): `"html"`, `"md"`, `"zip"`, or `"pdf"`

---

## Artifact Workflow (Step-by-Step)

Here's the standard workflow when a user asks you to create something:

### 1. Understand the Request
The user says something like "make a landing page for my SaaS" or "create slides for my talk."

### 2. Initialize Workspace (if needed)
```bash
cargo run -p od-cli -- init /path/to/workspace
```

### 3. Create the Artifact via MCP
Call `artifact_create` with the appropriate mode and visual brief:
- **Landing pages, articles, marketing**: mode `html`, brief `editorial`
- **Portfolios, showcases, slides**: mode `slides`, brief `studio`
- **Dashboards, tools, docs**: mode `docs` or `html`, brief `workbench`

### 4. Edit the Generated Files
Read the generated files, then edit them with your content. Follow the skill instructions in `skills/<mode>/SKILL.md`:
- Style through `--od-*` CSS custom properties (design tokens)
- Use `.od-*` class primitives and recipes
- Keep everything offline-friendly (no remote fonts, no CDN images)
- Use pure-CSS placeholders for images

### 5. Preview via MCP
Call `artifact_preview` — the native window opens and live-reloads as you edit.

### 6. Iterate
Edit files → preview auto-refreshes → repeat until the user is happy.

### 7. Export (when done)
Call `artifact_export` with the desired format. Generate `handoff.md` so the next session has context.

---

## Visual Briefs Reference

When choosing a `visualBrief`, follow these constraints from `skills/visual-briefs.md`:

### Editorial (default)
- Warm paper-like magazine feel, driven by typography and whitespace
- 3 colors + 1 accent maximum; "seven usually isn't"
- Use `--od-bg-canvas`, `--od-bg-surface`, `--od-text-*`, `--od-accent-solid`
- Layout: `.od-container` → `.od-section` → `.od-artifact` → `.od-doc`
- At most one primary CTA

### Studio
- Gallery wall / design critique board aesthetic
- Each page/slide is a quiet stage for a single idea
- Large whitespace, 16:9 slides via `.od-slide` and `.od-slide__inner`
- Media framed with `.od-frame` at fixed ratios
- Minimal accent used only for numbering/progress

### Workbench
- Clear information hierarchy; compact but not cramped
- Low-contrast neutral backgrounds
- Use `.od-grid`, `.od-cluster`, `.od-split` layout patterns
- Components: `.od-dashboard`, `.od-card`, `.od-button`, `.od-input`, `.od-badge`
- Interactive targets ≥ 40px; semantic colors via `data-tone` on badges only

---

## Universal Guardrails

These apply to **all** artifacts regardless of mode or brief:

- Use **only** `--od-*` design tokens; never hardcode raw hex colors outside `:root`.
- Ensure **WCAG AA** contrast on all text.
- `:focus-visible` must be visible on every interactive element.
- Respect `prefers-reduced-motion`.
- **No remote resources** — no Google Fonts, no CDN images, no external scripts.
- Artifact must open cleanly by double-clicking the HTML file (offline).
- **No JSX/TSX, Tailwind, shadcn/ui, or CDN UI kits** in generated artifacts.
- JavaScript must be minimal and only for real interaction (not for rendering).

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `odl: command not found` | Install dir not on PATH — use the absolute binary path, or add the dir printed by the install script |
| MCP tools not appearing in agent | Restart the agent after `odl setup`; run `odl setup --dry-run` to verify what was written |
| MCP tools flaky / server dies when preview opens | Check the config is **not** using `cargo run` (recompile output corrupts the stdio JSON-RPC stream) — rerun `odl setup --force`; `scripts/mcp_proxy.py` is a stdio-isolating debug proxy for diagnosis |
| Preview window doesn't open | Ensure `externalBrowser` is `false`; check the artifact directory exists; a `preview_crashed` error includes the tail of `<dir>/.odl/preview.log` |
| Preview doesn't live-reload | `watch` must be `true` (default); check file watcher limits on your OS |
| `link: extra operand` (Windows, source build) | Use `powershell -File scripts/build.ps1 build --release` instead of `cargo build` |

---

## Important: Do NOT

- ❌ Open a system/external browser yourself (`start`, `xdg-open`, `open`, Playwright, Cursor Simple Browser, etc.)
- ❌ Start a static file server (`python -m http.server`, `npx serve`, etc.)
- ❌ Set `externalBrowser: true` unless the user explicitly asks
- ❌ Use Tailwind, shadcn, Bootstrap, or any CDN UI framework in generated artifacts
- ❌ Reference remote fonts, images, or scripts
- ❌ Add build steps or dev servers to artifacts
