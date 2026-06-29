# Extensions

Editor integrations are thin adapters, not the product core.

Targets:

- Codex MCP/client workflow
- Cursor extension or command integration
- Zed extension
- VS Code extension if needed

Each extension should call `odl` or MCP tools and render/attach the same artifacts. No extension should own artifact storage, skill format, or model routing.
