---
name: preview-via-mcp
mode: workflow
description: Preview Open Design artifacts by calling the MCP tool artifact_preview — never open a system browser yourself. Use whenever the user asks to preview, open, show, or live-reload an artifact.
---

# preview-via-mcp

When the user wants to preview an Open Design artifact, use the Open Design MCP tool — not a browser you launch yourself.

## Required preview path

1. Call MCP tool `artifact_preview` with the artifact root as `dir`.
2. Prefer defaults: omit `externalBrowser` (or set `false`) and leave `watch` true so the persistent `odl preview` window opens and live-reloads.
3. After edits, call `artifact_preview` again only if the window is not already running; otherwise rely on live reload.

## Do not

- Do **not** open a system/external browser yourself (`start`, `xdg-open`, `open`, Playwright, Cursor Simple Browser, etc.).
- Do **not** set `externalBrowser: true` unless the user explicitly asks for the system browser.
- Do **not** start a separate static file server or `python -m http.server` for preview.

## Why

`artifact_preview` spawns the MCP-managed preview window (`odl preview`): persistent, live-reloading, and the product's intended agent preview path. Opening a browser yourself bypasses that window and breaks the live-reload loop.

## CLI note

Humans and scripts may still run `odl preview <dir>` directly. Coding agents with Open Design MCP available must use `artifact_preview` instead.
