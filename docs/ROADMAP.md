# Roadmap

## M0: Repo Contract

- PRD and architecture docs.
- Rust workspace scaffold.
- Built-in skill placeholders.
- Minimal CLI commands.

## M1: Local Artifact Loop

- `odl init`
- `odl new html/docs/slides`
- native preview shell opens artifact folder
- file watcher refreshes preview
- handoff file generation

## M2: Agent Bridge

- MCP server exposes create/preview/export/handoff.
- Handoff prompts for Codex, Claude Code, OpenCode, Cursor, and Zed.
- Optional external command runner for agent-created artifacts.

## M3: Built-In Generation

- BYOK OpenAI-compatible provider.
- Generate HTML, docs, and slides directly.
- Prompt templates and mode-specific quality checks.

## M4: Export

- Self-contained HTML.
- Markdown.
- PDF from preview.
- ZIP artifact bundle.

## M5: Thin Plugins

- Cursor command integration.
- Zed command/panel integration.
- VS Code preview panel if useful.
- Codex/Claude Code MCP install helpers.
