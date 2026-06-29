# Open Design Lite

Open Design Lite, working binary name `odl`, is a lightweight local design assistant for fast artifact work:

- create or polish a single HTML page
- polish Markdown docs
- draft lightweight HTML slides
- preview instantly in a native shell
- hand off the result to Codex, Claude Code, OpenCode, Cursor, Zed, or another agent

This project intentionally does not start as a heavy web app. The product shape is a small local binary plus a native preview shell. IDE/editor plugins are thin clients over the same local kernel.

## Direction

We choose **binary shell first, plugin second**.

The shell gives us one fast, predictable runtime. Plugins for Codex, Cursor, Zed, VS Code, and others can come later as adapters that call the same CLI/MCP surface. This avoids rebuilding the product separately for every editor.

## Repository Map

```text
apps/
  shell/              Native preview shell placeholder
  extensions/         Thin editor/plugin adapters placeholder
crates/
  od-core/            Artifact, workspace, skill, and run primitives
  od-cli/             `odl` command entrypoint
  od-mcp/             MCP tool surface for coding agents
  od-preview/         Preview/export integration boundary
docs/
  PRD.md              Product requirements (+ PRD_zh.md)
  ARCHITECTURE.md     Technical architecture (+ ARCHITECTURE_zh.md)
  ROADMAP.md          Milestones (+ ROADMAP_zh.md)
  adr/                Architecture decisions (+ *_zh.md)
skills/
  html-page/          Built-in HTML artifact skill
  docs-polish/        Built-in docs polishing skill
  slides-html/        Built-in HTML slides skill
templates/
  html-page/          Starter HTML templates
  slides/             Starter slide templates
```

## MVP Promise

Start fast, generate fast, preview fast. The first useful version should feel closer to opening a tiny local utility than booting a platform.

## Initial Commands

```powershell
cargo run -p od-cli -- init .odl-demo
cargo run -p od-cli -- new html .odl-demo/hello
cargo run -p od-cli -- preview .odl-demo/hello
```

The current code is only a scaffold. The product contract is documented first so implementation can stay small.
