# Agent Instructions

This repository is for a lightweight, local-first design assistant. Keep changes small, fast, and boring unless the product goal requires otherwise.

## Product Principles

- Prefer a local binary and static assets over a web server.
- Keep the kernel independent from any one editor or coding agent.
- Treat plugins as thin adapters over CLI/MCP, not as the core product.
- Store artifacts as plain files on disk.
- Optimize first-run and time-to-preview over ecosystem breadth.

## Engineering Rules

- Do not commit upstream research clones; `external/` is ignored.
- Do not add a heavy framework without an ADR.
- Keep runtime dependencies minimal until the MVP path is proven.
- Built-in skills should be filesystem directories with a readable `SKILL.md`.
- Generated artifacts should be inspectable and handoff-friendly.

## Current Decision

The primary product surface is a native binary shell (`odl`) with a local preview. MCP and editor extensions are secondary surfaces that drive the same kernel.
