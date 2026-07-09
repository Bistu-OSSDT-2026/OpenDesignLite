# Agent Instructions

This repository is for a lightweight, local-first design assistant. Keep changes small, fast, and boring unless the product goal requires otherwise.

## Product Principles

- Prefer a local binary and static assets over a web server.
- Keep the kernel independent from any one editor or coding agent.
- MCP and the CLI are the integration contract; agents drive the kernel through them.
- Store artifacts as plain files on disk.
- Optimize first-run and time-to-preview over ecosystem breadth.

## Engineering Rules

- Do not commit upstream research clones; `external/` is ignored.
- Do not add a heavy framework without an ADR.
- Keep runtime dependencies minimal until the MVP path is proven.
- Built-in skills should be filesystem directories with a readable `SKILL.md`.
- Generated artifacts should be inspectable and handoff-friendly.

## Current Decision

The primary product surface is the **MCP tool surface**: coding agents drive `od-core` over MCP, and `artifact_preview` auto-launches a persistent, live-reloading preview window. The `odl` CLI is a scripting convenience over the same kernel. There is no separate chat/preview shell app.

## Preview (agents)

When previewing an artifact, call MCP `artifact_preview` with the artifact `dir`. Do **not** open a system/external browser yourself (`start`, `xdg-open`, `open`, Playwright, etc.). Keep defaults (`externalBrowser` false, `watch` true) so the MCP-managed `odl preview` window is used. See `skills/preview-via-mcp/SKILL.md`.
