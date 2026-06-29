# Native Shell

The shell is the primary product surface.

Planned shape:

- one small binary window
- left side: prompt, mode, selected files, agent status
- right side: instant preview
- no Next.js server in production
- system WebView for HTML/slides preview
- Markdown rendered locally

Implementation target is still open, but the current preference is a Rust binary with a thin WebView layer. Tauri is acceptable only if runtime stays static and fast.
