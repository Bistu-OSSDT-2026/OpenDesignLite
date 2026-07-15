# Open Design Lite — Agent 使用规则

本文件面向**把本仓库当 MCP server 使用**的编码 Agent（dogfood/演示场景）。改动本仓库代码的工程规则见 [AGENTS.md](AGENTS.md)；在用户自己项目中使用请走 `odl setup`（见 [AGENT_SETUP.md](AGENT_SETUP.md)）。

## 前置

仓库根目录的 `.mcp.json` 指向 `target/release/odl.exe`——首次使用前先构建：

```powershell
cargo build --release
# Windows link.exe 报错时：powershell -File scripts/build.ps1 build --release
```

（Unix 环境把 `.mcp.json` 的 command 改为 `target/release/odl`，或直接运行 `target/release/odl setup`。）

## 预览规则（强制）

- 创建产物用 `artifact_create`；它**默认自动弹出预览窗口**（`autoPreview: true`）。若窗口未出现，调用 `artifact_preview`（参数 `dir` 为产物目录绝对路径）。
- **禁止**自行用系统浏览器/静态服务器打开产物：不要 `start` / `xdg-open` / `open` / Playwright / Simple Browser / `python -m http.server`。预览必须走 MCP，才能保持 live-reload 闭环。
- 同一目录重复调用 `artifact_preview` 是安全的（返回 `alreadyRunning: true`，不会弹第二个窗口）。
- 预览窗口按产物类型固定尺寸（slides 1280×720 16:9；html/docs 1366×768），所见即部署后所得。
- 预览出问题先看 `<产物目录>/.odl/preview.log`。

## 产物规则

- 只用 `--od-*` design token 与 `.od-*` class；禁止 Tailwind/shadcn/CDN UI kit/远程字体图片脚本。
- 详细生成规范见 `skills/<mode>/SKILL.md` 与 `skills/visual-briefs.md`。
- slides 每页内容必须收在 16:9 内——宁可拆页。导出 PDF 固定 16:9。
