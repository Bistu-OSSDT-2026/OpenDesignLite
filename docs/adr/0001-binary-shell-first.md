# ADR 0001: Binary Shell First

## Status

Accepted.

## Context

The product should be lightweight, fast to launch, and useful for small daily design artifacts. Two candidate surfaces were considered:

1. A standalone binary shell that previews artifacts and connects to agents through CLI/MCP.
2. Editor-specific plugins, such as Codex/Cursor/Zed side panels.

## Decision

Build the standalone binary shell first. Use CLI and MCP as stable integration contracts. Build editor plugins later as thin adapters.

## Consequences

Positive:

- One core runtime.
- Faster MVP path.
- Works from any editor or terminal.
- Avoids plugin API fragmentation.
- Keeps artifact storage and preview behavior consistent.

Negative:

- We still need thin plugins later for best editor ergonomics.
- The shell must provide enough UX to be worth opening.
- We need to choose a native shell stack carefully to avoid recreating a heavy web app.

## Notes

Using a system WebView for preview is acceptable. The rejected part is a heavy web platform with a dev server and complex browser app runtime.
