# Agent Setup Guide

> **Instructions for coding agents to automatically install, build, and configure Open Design Lite.**
>
> Read this file when a user asks you to set up Open Design Lite, create a design artifact, or preview HTML/Markdown/Slides via MCP.

---

## What This Project Is

Open Design Lite (`odl`) is a **local-first design assistant** that you (the coding agent) drive through MCP tools. You call `artifact_create` to make HTML pages / Markdown docs / slides, `artifact_preview` to open a live-reloading preview window, and `artifact_export` to package the result. There is no web server, no cloud, no sign-up — everything is local.

---

## Quick Auto-Install

Run these steps to get Open Design Lite ready on the user's machine:

### Step 1: Check Prerequisites

Verify Rust is installed. If not, guide the user to install it:

```bash
rustc --version
cargo --version
```

If missing: direct user to https://rustup.rs/ or run `winget install Rustlang.Rustup` (Windows).

### Step 2: Clone and Build

```bash
git clone https://github.com/Bistu-OSSDT-2026/OpenDesignLite.git
cd OpenDesignLite
cargo build --release
```

On Windows, if linking fails with `link: extra operand`, use the helper:
```powershell
powershell -File scripts/build.ps1 build --release
```

### Step 3: Verify the Build

```bash
cargo run -p od-cli -- --help
```

Expected: help text listing subcommands (`init`, `new`, `preview`, `export`, `handoff`, `skill`, `mcp`).

### Step 4: Configure MCP

Add this MCP server configuration to the user's agent config file. Use the **absolute path** to the cloned repository.

**Template** (fill in `$REPO_PATH` with the actual clone path):

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "cargo",
      "args": ["run", "-p", "od-cli", "--", "mcp"],
      "cwd": "$REPO_PATH"
    }
  }
}
```

**Agent-specific locations:**

| Agent | Config File |
|-------|-------------|
| Claude Code | `~/.claude/claude_desktop_config.json` or project `.mcp.json` |
| Codex / OpenCode | `opencode.json` → `mcp` section |
| Cursor | `~/.cursor/mcp.json` |
| Zed | `settings.json` → `context_servers` section |
| ZCode | `.zcode/mcp.json` or workspace MCP settings |

After adding the config, **restart the agent** for MCP tools to appear.

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

**What it does:** Creates a directory with starter files (`index.html`, `assets/od-design.css`, `manifest.json`, `handoff.md`) based on the chosen mode and visual brief.

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
| `cargo: command not found` | Rust not installed. Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `link: extra operand` (Windows) | Use `powershell -File scripts/build.ps1 build --release` instead of `cargo build` |
| MCP tools not appearing in agent | Restart the agent after adding MCP config; verify `cwd` is an absolute path |
| Preview window doesn't open | Ensure `externalBrowser` is `false`; check that the artifact directory exists |
| Preview doesn't live-reload | `watch` must be `true` (default); check file watcher limits on your OS |

---

## Important: Do NOT

- ❌ Open a system/external browser yourself (`start`, `xdg-open`, `open`, Playwright, Cursor Simple Browser, etc.)
- ❌ Start a static file server (`python -m http.server`, `npx serve`, etc.)
- ❌ Set `externalBrowser: true` unless the user explicitly asks
- ❌ Use Tailwind, shadcn, Bootstrap, or any CDN UI framework in generated artifacts
- ❌ Reference remote fonts, images, or scripts
- ❌ Add build steps or dev servers to artifacts
