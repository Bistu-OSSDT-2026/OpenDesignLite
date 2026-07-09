# Open Design Lite

Local-first design assistant (`odl`) that plugs into your coding agent over **MCP**. Describe what you want in one sentence; the agent calls Open Design Lite's tools to create HTML pages, Markdown docs, or lightweight HTML slides, and a live preview window pops open and refreshes as the agent iterates.

Design defaults stay lightweight and framework-agnostic: CSS variables, static recipes, and plain files instead of a bundled UI runtime.

**Docs**: [docs/README.md](docs/README.md) (中文)

## How it works

1. Configure the `open-design-lite` MCP server once in your agent (opencode / Codex / Claude Code / Cursor / Zed…).
2. Tell the agent what you want.
3. The agent calls the MCP tools (`artifact_create`, `artifact_preview`, …) to build the artifact and auto-open a preview.
4. Keep chatting with the agent to tweak; the persistent preview window live-reloads on every change.

No separate app, no chat/preview shell to install — the agent is your UI, the preview is just a window.

> MCP lives in `crates/od-mcp`. Start it with `odl mcp` (JSON-RPC over stdio). `artifact_create` / `artifact_preview` / `artifact_handoff` / `artifact_export` are wired. Agent config docs and live client verification are the remaining M2 polish. See [docs/product/roadmap.md](docs/product/roadmap.md).

## Quick Start (scripting)

The `odl` CLI is a scripting convenience over the same kernel:

```powershell
cargo run -p od-cli -- init .odl-demo
cargo run -p od-cli -- new html .odl-demo/hello
cargo run -p od-cli -- preview .odl-demo/hello
```

## Repository

```text
crates/od-core    od-cli    od-mcp    od-preview
skills/           templates/
docs/             ← start here
```

## Direction

MCP-first: agents drive design via MCP, and the preview auto-launches and live-reloads. The CLI is a scripting convenience over the same kernel. The design system lives in core as tokens and recipes, not as a React/Tailwind/shadcn runtime.
