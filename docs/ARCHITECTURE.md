# Architecture

## Decision

Build a lightweight local kernel with a native binary shell. Use CLI/MCP as the stable integration contract. Treat editor plugins as optional adapters.

## Runtime Layers

```text
Native Shell / CLI / MCP / Extensions
              |
              v
        od-core kernel
              |
   +----------+----------+
   |          |          |
skills     artifacts   preview/export
   |          |          |
plain files on disk     system WebView / Markdown renderer
```

## Why Binary Shell First

The user experience target is fast and local. A binary shell:

- avoids booting a web app stack
- gives us one preview implementation
- can be launched from any editor or terminal
- keeps the kernel independent from Codex/Cursor/Zed APIs

## Why Not Plugin First

A plugin-first product would fragment immediately:

- Cursor, VS Code, Zed, Codex, and Claude Code have different extension surfaces.
- Preview panels behave differently across editors.
- Release and permissions models differ.
- The core product would become trapped inside the first plugin we build.

Instead, plugins should call `odl` or MCP and display the same artifact.

## Artifact Model

An artifact is a folder:

```text
my-artifact/
  index.html | doc.md | slides.html
  assets/
  manifest.json
  handoff.md
```

Everything should remain human-readable and agent-readable.

## Skill Model

A skill is a folder:

```text
skills/html-page/
  SKILL.md
  templates/
```

The first version only needs readable Markdown instructions plus optional templates. No registry or marketplace until the built-ins are excellent.

## Agent Integration

MVP supports three levels:

- Handoff file: produce prompts and context for external agents.
- CLI: `odl new`, `odl preview`, `odl export`, `odl handoff`.
- MCP: expose artifact creation/preview/export/handoff tools.

Direct model calls can be added after the local workflow is proven.

## Preview

Preview is local and file-based:

- HTML/slides: system WebView.
- Markdown: native renderer or converted static HTML in WebView.
- Future PDF: print/export from WebView.

No dev server is required for generated artifacts.
