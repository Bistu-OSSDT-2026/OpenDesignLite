# Open Design Lite

Local-first design assistant (`odl`): HTML pages, Markdown docs, and lightweight HTML slides — create, preview, and hand off to coding agents.

Design defaults stay lightweight and framework-agnostic: CSS variables, static recipes, and plain files instead of a bundled UI runtime.

**Docs**: [docs/README.md](docs/README.md) (中文)

## Quick Start

```powershell
cargo run -p od-cli -- init .odl-demo
cargo run -p od-cli -- new html .odl-demo/hello
cargo run -p od-cli -- preview .odl-demo/hello
```

Scaffold only; see [docs/product/roadmap.md](docs/product/roadmap.md) for milestones.

## Repository

```text
crates/od-core    od-cli    od-mcp    od-preview
apps/shell        extensions/
skills/           templates/
docs/             ← start here
```

## Direction

Binary shell first, thin editor plugins later. CLI and MCP are the stable integration surface. The design system belongs in core as tokens and recipes, not as React/Tailwind/shadcn runtime.
