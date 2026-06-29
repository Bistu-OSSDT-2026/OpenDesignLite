# PRD: Open Design Lite

## Summary

Open Design Lite is a lightweight local design assistant for quickly creating, polishing, previewing, and handing off small design artifacts: HTML pages, Markdown docs, and lightweight slide decks.

The product is not a web platform. It should feel like a fast local utility that can be opened from a terminal, editor, or native shell.

## Problem

Heavy AI design tools are slow to start, complex to configure, and often overbuilt for common daily work:

- make a quick HTML mock
- polish a docs page
- draft a slide deck
- preview and tweak the result
- hand it to a coding agent for implementation

The user wants speed and low ceremony more than an all-in-one design platform.

## Goals

- Launch quickly as a local binary.
- Create a previewable artifact in under a minute.
- Keep artifacts as normal files.
- Work with external agents through CLI/MCP/handoff.
- Provide a native preview shell without requiring a browser tab or web dev server.
- Make the first three modes excellent: HTML, Docs, Slides.

## Non-Goals

- Full Figma replacement.
- Multi-user collaboration.
- Plugin marketplace.
- Cloud accounts or sync.
- Heavy project management.
- General coding agent replacement.
- Full visual editor in v1.

## Primary Users

- Builders who use Codex, Claude Code, OpenCode, Cursor, or Zed.
- PMs/founders who need fast docs, mockups, and decks.
- Engineers who want previewable artifacts before implementation.
- Designers who want small generated deliverables without opening a large design suite.

## Product Shape

Open Design Lite has one core runtime and multiple entry points:

- Native shell: primary UI and preview surface.
- CLI: scriptable local workflow.
- MCP server: agent integration surface.
- Editor extensions: thin wrappers over CLI/MCP.

## MVP Modes

### HTML Page

Input: prompt, optional files, optional style notes.

Output:

- `index.html`
- optional `assets/`
- `handoff.md`

Preview: native shell WebView.

### Docs Polish

Input: Markdown or text.

Output:

- `doc.md`
- optional `handoff.md`

Preview: native Markdown rendering.

### Slides

Input: topic, outline, pasted notes, or docs.

Output:

- `slides.html`
- optional `handoff.md`

Preview: native shell WebView.

## Core Workflow

1. User opens shell or runs `odl`.
2. User picks mode: HTML, Docs, or Slides.
3. User gives a prompt or file.
4. Kernel creates a plain-file artifact workspace.
5. Preview opens immediately when the first file exists.
6. User asks for edits.
7. User exports or hands off to an external agent.

## Integration Requirements

The first-class integration contract is not an editor plugin. It is:

- CLI commands
- MCP tools
- plain files
- generated handoff prompts

Plugins should be thin:

- open preview panel
- send selected text/file to `odl`
- call MCP tools
- display artifact status

## Success Metrics

- Cold start feels instant for a small local utility.
- New artifact workspace can be created with one command.
- HTML preview works without project setup.
- Docs polish works without a browser.
- Slides preview works without a build step.
- Handoff to Codex/OpenCode/Claude Code is understandable from files alone.

## Open Questions

- Should the native shell be Tauri, raw Wry, or another Rust WebView wrapper?
- Should model calls live inside `odl` v1 or should v1 only orchestrate external agents?
- How much inline editing should exist before editor plugins?
- What is the smallest acceptable export set: HTML/MD only, or PDF too?
