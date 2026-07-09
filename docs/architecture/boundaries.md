# 模块边界

防止 MCP、CLI、preview 各自长出一套逻辑。

## 归属表

| 能力 | 所有者 | 不应出现在 |
|------|--------|------------|
| 产物路径、类型、manifest 解析 | `od-core` | CLI/MCP 重复实现 |
| Design token、layout primitive、recipe 语义 | `od-core` | templates/skills 各自发明 |
| 文件监视与 WebView 生命周期 | `od-preview` | `od-cli` / `od-mcp` 内嵌 UI |
| 子命令解析与用户输出 | `od-cli` | `od-mcp` |
| MCP JSON-RPC 与 tool schema | `od-mcp` | `od-cli` |
| 技能文案与模板 | `skills/` + `templates/` | 硬编码在 Rust |
| 具体前端框架 adapter | 后续 adapters（导出目标） | `od-core` 直接依赖 |

## 集成契约（变更需同步）

以下变更前必须在 PR 中 @ 全员并更新对应 spec：

1. **产物目录布局** — `manifest.json` 字段、主文件命名
2. **Skill front matter** — `SKILL.md` 头部字段
3. **CLI** — 子命令名、flags、退出码
4. **MCP** — tool 名、参数 schema
5. **可预览文件检测顺序** — 当前：`index.html` → `slides.html` → `doc.md`
6. **导出格式** — `html` `md` `zip` `pdf`
7. **Design kernel 语义** — token 命名、primitive 名称、recipe 名称

## 数据流：创建到预览

```text
编码 Agent（MCP） | od-cli
    → od-core::Artifact::new
        → fs: 写主文件 + handoff.md
    → od-preview::open(artifact.root)
        → 解析主文件路径
        → WebView load（自动弹出常驻窗口）
        → watcher → reload（Agent 后续改文件即刷新）
```

## 数据流：交接

```text
od-core 或 skill 逻辑
    → 写 handoff.md（意图、约束、建议下一步）
User 将目录或 handoff 交给外部 Agent
    → Agent 直接编辑文件 或 调 odl / MCP
```

## 反模式

- 在 MCP/CLI 层另存一套 artifact 路径数据库（真相在磁盘）
- 为预览启动 `npm run dev`
- 自研独立的对话/预览壳层 app（Agent 即界面）
- 在内核引用任何编辑器/Agent SDK
- 未 ADR 引入重型前端框架
- 在 `od-core` 直接依赖 React、Tailwind、Radix、shadcn/ui、Lit 或 Web Components runtime
- 每个 template 自己定义一套不兼容的颜色、间距、圆角、阴影命名
- 把 design system 做成固定页面模板库，而不是可组合 token / recipe / pattern

## 与上游调研的关系

曾评估 fork `nexu-io/open-design` 等方案，**已否决**。见 [research/kernel-candidates.md](../research/kernel-candidates.md)。当前路线：自研轻量 Rust 内核，仅借鉴概念。

Design system 调研见 [research/design-system-kernel.md](../research/design-system-kernel.md)。当前路线：极轻、框架无关的 design kernel；具体 UI kit 只作为 adapter 或生成目标。
