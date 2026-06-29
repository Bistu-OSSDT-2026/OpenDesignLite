# Open Design Kernel Research

日期：2026-06-29

目标：做一个 Claude Design 的开源对应版本。优先调研现有 GitHub 项目是否能作为 kernel；如果内核合适，就 fork/套壳，重点做更顺滑的 OpenCode / Claude Code / Codex / Zed Code 等 agent 接入；如果不合适，再自研 kernel。

## 结论

我建议把 `nexu-io/open-design` 作为第一候选内核继续验证。它不是简单的 Artifacts clone，而是已经围绕「本地 daemon + desktop/web UI + agent CLI adapter + skills + DESIGN.md + sandbox preview + export」建成了一整套平台，方向和我们想做的 Open Design 高度重合。

第二候选是 `OpenCoworkAI/open-codesign`。它更轻、更干净、MIT license，而且产品取向也接近 Claude Design；但它的内核主要依赖 `pi-agent-core/pi-coding-agent`，对 Codex / OpenCode / Claude Code 这类外部 CLI 的适配不是第一原则。如果我们想更快做一个“自己掌控、轻量、可重构”的 kernel，它是好底座；如果目标是一开始就丝滑接入各种 coding agent，`nexu-io/open-design` 更接近成品。

`ZSeven-W/openpencil` 值得单独学习它的 canvas/vector/design-as-code/MCP 设计，但它不是 Claude Design 等价内核。它更像 AI-native vector/Figma/Pencil 方向，可以作为后续“画布层”或 `.op` JSON 文件模型的参考。

`Nutlope/llamacoder`、`13point5/open-artifacts`、LibreChat Artifacts 等更像 Claude Artifacts 代替品，不足以覆盖 Claude Design 的完整产品面。

## 已拉到本地的候选

本轮已浅克隆：

- `external/open-design` from https://github.com/nexu-io/open-design
- `external/open-codesign` from https://github.com/OpenCoworkAI/open-codesign
- `external/openpencil` from https://github.com/ZSeven-W/openpencil
- `external/llamacoder` from https://github.com/Nutlope/llamacoder

当前目录 `D:\projects2\opendesign` 还不是 Git 仓库；这些候选先放在 `external/` 里作为调研素材。

## 我们真正要对标的功能面

Claude Design 不应理解成“一个聊天窗口生成 HTML”。更合理的目标拆分是：

- Prompt to design artifact：网页、移动端、dashboard、deck、报告、营销素材等。
- Design system first：能摄取、生成、维护 `DESIGN.md` 或等价品牌契约。
- Iteration loop：评论、点选、局部修改、tweaks/sliders、可中断、可继续。
- Sandboxed preview：安全预览 HTML/JSX/React artifact，并能捕获 console/error/截图。
- Export：HTML、PDF、PPTX、ZIP、图片等。
- Agent handoff：能把设计产物、tokens、组件、意图说明交给 coding agent。
- Local-first / BYOK：用户自己的模型 key、CLI、文件系统和 workspace。
- Multi-agent/CLI adapters：Claude Code、Codex、OpenCode、Cursor、Gemini、Copilot、Qwen 等。

这也是为什么单纯 Artifacts clone 不够。

## 候选评分

| 项目 | 定位 | License | 活跃度 | Kernel 适配度 | 备注 |
|---|---|---:|---:|---:|---|
| `nexu-io/open-design` | 本地优先 Claude Design alternative，daemon/web/desktop/CLI/MCP/skills/design systems | Apache-2.0 | 很高，2026-06-29 仍在 push，latest release `open-design-v0.12.0` | 9/10 | 最像我们想要的完整底座；体量大，复杂度和品牌/路线分叉成本也最大。 |
| `OpenCoworkAI/open-codesign` | Electron 本地设计 agent，BYOK，多模型，prompt to artifact/deck/PDF | MIT | 高，2026-06-28 push，latest release `v0.2.1` | 7.5/10 | 轻量、可控、MIT；但外部 CLI adapter 不是主架构，更多依赖 pi-agent-core。 |
| `ZSeven-W/openpencil` | AI-native vector design tool，design-as-code，canvas，MCP | MIT | 高，2026-06-25 push，无 latest release API 记录 | 6/10 | 很适合借鉴 canvas/vector/DSL/MCP，不适合作为 Claude Design 主 kernel。 |
| `Nutlope/llamacoder` | Claude Artifacts-like app generator | MIT | 中，2026-05-08 push | 3/10 | Sandpack + Next.js demo，适合参考 Artifacts preview，不够 Claude Design。 |
| `13point5/open-artifacts` | 早期 Claude Artifacts clone | MIT | 低，repo archived | 1/10 | 已归档，不建议采用。 |
| `LibreChat` | 通用 AI chat，自带 Artifacts 等能力 | MIT | 很高 | 2/10 | 强在聊天平台，不是 design kernel。 |

## 第一候选：nexu-io/open-design

仓库：https://github.com/nexu-io/open-design

本地路径：`external/open-design`

元数据：

- Stars：约 72.5k
- Forks：约 8.2k
- License：Apache-2.0
- 创建时间：2026-04-28
- 最近 push：2026-06-29
- Latest release：`open-design-v0.12.0`，2026-06-26
- 技术栈：pnpm monorepo，Node `~24`，Next.js web，Electron desktop，daemon app，TypeScript packages。

关键结构：

- `apps/daemon`：本地 daemon 和 `od` CLI，负责 `/api/*`、agent spawning、skills、design systems、artifacts、exports。
- `apps/web`：Next.js 16 UI。
- `apps/desktop`：Electron shell。
- `packages/contracts`、`packages/platform`、`packages/sidecar-*`：跨进程/跨 app 协议和平台层。
- `skills/`：本地技能目录，本轮统计约 157 个目录。
- `design-systems/`：设计系统目录，本轮统计约 151 个目录。
- `docs/agent-adapters.md`：明确把外部 coding agent CLI 当成核心运行时。
- `apps/daemon/src/runtimes/registry.ts`：内置 agent defs 包括 Claude、Codex、OpenCode、Cursor、Gemini、Qwen、Qoder、Copilot、Aider、Devin、Kimi、Hermes 等。

非常符合我们目标的点：

- 核心设计哲学就是“不要重写 coding agent loop，而是适配用户已有 CLI”。
- 已经有 `od mcp install <agent>`、CLI/MCP、agent adapter、daemon API 这类可嵌入边界。
- Skill 协议兼容 Claude Code `SKILL.md`，并扩展 `od:` metadata 用于 preview、inputs、parameters、outputs、design system。
- `DESIGN.md` 是第一类对象，并且设计系统/skill/craft/reference 是体系化的。
- 支持 sandboxed iframe preview，支持导出 PDF / image / HTML / PPTX / ZIP / Markdown / JSX 等方向。
- 文档明确提到 OpenCode、Codex、Claude Code 的适配路径。

风险：

- 体量很大。浅克隆后 10k+ 文件，迁移/裁剪/换壳成本不会低。
- Node 24 + pnpm 10.33 + Electron + better-sqlite3 等依赖会让 Windows native 开发体验偏重。
- Apache-2.0 没问题，但如果我们想做极简 MIT kernel，license 和贡献治理要提前想清楚。
- 项目已经叫 Open Design，和我们的命名/品牌完全重叠。若直接 fork，需要决定是 upstream fork、hard fork、还是只吸收架构。
- 功能覆盖很广，可能带来“我们还没想清楚但它已经做了很多”的产品复杂度。

我的判断：值得直接 fork/继续跑通。下一步应该不是马上大改 UI，而是先做 smoke run：安装、启动 daemon/web、检测本机 Codex/OpenCode/Claude Code、生成一个 prototype、导出 HTML/PDF，再做一次“handoff to Codex”的闭环。

## 第二候选：OpenCoworkAI/open-codesign

仓库：https://github.com/OpenCoworkAI/open-codesign

本地路径：`external/open-codesign`

元数据：

- Stars：约 7.0k
- Forks：约 733
- License：MIT
- 创建时间：2026-04-18
- 最近 push：2026-06-28
- Latest release：`v0.2.1`，2026-05-23
- 技术栈：pnpm + Turborepo，Electron desktop，TypeScript packages，`pi-agent-core` / `pi-coding-agent` / `pi-ai`。

关键结构：

- `packages/core/src/agent-session.ts`、`agent.ts`：agent loop/session。
- `packages/core/src/tools/`：`ask`、`preview`、`scaffold`、`skill`、`tweaks`、`done`、`text-editor`、UI kit parity tools。
- `packages/runtime`：iframe preview runtime，支持 HTML/JSX/TSX 分类和 overlay/tweaks bridge。
- `docs/VISION.md`：目标清楚，local-first、BYOK、DESIGN.md、files are real。
- `docs/v0.2-plan.md`：中文计划很细，说明 v0.2 主线是 pi function-tool loop + workspace delta stream。

优点：

- License 是 MIT，后续商业/开源组合最省心。
- 代码规模比 `open-design` 小，产品边界更容易被我们掌控。
- 已有设计 agent 的关键工具：preview、tweaks、scaffold、skill、done、visual parity。
- 文档非常接近我们“先从 docs 开始”的思路，尤其 v0.2 plan 可直接作为讨论材料。
- 对设计质量的验证思路比普通 Artifacts clone 强，包括 image to UI kit、boolean visual parity judge。

不足：

- 它的核心选择是 `pi-coding-agent`，不是“外部 agent CLI adapter pool”。如果我们的核心卖点是顺滑接入 opencode/cc/codex/zcode，需要补一层 adapter/handoff。
- v0.2 文档里明确 MCP 是 non-goal；这和我们希望可嵌入外部 agent 的方向有冲突。
- 相比 `open-design`，设计系统、skills、agent platform 的生态面更小。

我的判断：如果 `open-design` 跑起来太重或产品路线太强势，`open-codesign` 是更适合 hard fork 成“我们的 kernel”的备选。它的 `packages/runtime`、`packages/core/src/tools`、prompt sections、tweaks 协议值得拆出来研究。

## 参考候选：ZSeven-W/openpencil

仓库：https://github.com/ZSeven-W/openpencil

本地路径：`external/openpencil`

元数据：

- Stars：约 3.6k
- Forks：约 362
- License：MIT
- 最近 push：2026-06-25
- 技术栈：Bun workspace，Vite/TanStack/React/Electron，MCP server，ACP，OpenCode SDK，Claude agent SDK，canvas/vector engine。

值得借鉴：

- `.op` JSON design-as-code 文件模型。
- canvas/vector engine：`packages/pen-core`、`pen-engine`、`pen-renderer`、`pen-react`。
- MCP server 和 CLI：外部 agent 可以读/改设计文件。
- concurrent agent teams 概念：复杂页面按空间区域并行生成。
- Figma import/export/clipboard 相关代码可能有用。

不适合当主 kernel 的原因：

- 它要解决的是 vector/canvas/Figma-like 工具，不是 artifact/deck/report/export-first 的 Claude Design 工作流。
- Bun + Zig native + canvas/vector 栈更复杂，偏向图形工具内核。
- 如果我们第一阶段目标是“更简单、更丝滑地接入 coding agents 生成 artifacts”，它会把问题域拉宽。

我的判断：不要拿它当主底座；把它当后续画布模式、设计 DSL、MCP design file server 的参考。

## 不建议作为主底座

### Nutlope/llamacoder

仓库：https://github.com/Nutlope/llamacoder

它是 Next.js + Sandpack 的 Claude Artifacts-like app generator，依赖 Together AI、CodeSandbox API、Neon/Prisma。适合看最小 artifact preview 和 app generation demo，但不具备 design system、desktop/local-first、agent adapter、export、handoff 这条主线。

### 13point5/open-artifacts

仓库：https://github.com/13point5/open-artifacts

repo 已 archived，且定位是早期 Claude.ai Artifacts clone。不建议继续投入。

### LibreChat

仓库：https://github.com/danny-avila/LibreChat

适合作为通用 AI chat/Artifacts/MCP 平台参考，但不是 design artifact kernel。直接复用会把我们拖到聊天平台问题域。

## 推荐路线

### 路线 A：fork `nexu-io/open-design`，先做产品壳和 agent 接入体验

适合目标：最快接近 Claude Design 对标功能，并强化 opencode / cc / codex / zcode 入口。

做法：

1. 保留 daemon + agent adapter + skills/design-systems + export pipeline。
2. 先换更简单的 shell：信息架构、首屏、agent picker、project creation、handoff flow。
3. 专门做 “Continue in Codex / OpenCode / Claude Code / Zed Code” 的一键路径。
4. 减少或隐藏不必要复杂度，把产品打磨成更简单的 Open Design。

风险：上游体量大，路线/品牌重叠，需要决定 fork 策略。

### 路线 B：以 `open-codesign` 为轻 kernel，自研 agent adapter 层

适合目标：我们想要更小、更可控、更 MIT 的 kernel。

做法：

1. 借用它的 preview runtime、tools、tweaks、design system prompt、visual parity。
2. 新增 agent adapter 层：Codex/OpenCode/Claude Code/Zed Code/ACP/MCP。
3. 把 storage/workspace/session 设计做轻，先支持 artifact/deck/export。

风险：要自己补外部 CLI 编排、MCP/CLI embeddability、skills registry 和更多 export/handoff。

### 路线 C：自研 kernel，只吸收设计

适合目标：我们发现两个项目都太重或路线冲突。

建议只在 smoke test 失败后再走这条路。自研时可以采用：

- `open-design` 的 adapter/spec 思路。
- `open-codesign` 的工具清单和 preview/tweaks runtime。
- `openpencil` 的 design-as-code/canvas/MCP 文件模型。

## 下一步验证清单

优先做 `open-design` smoke test：

1. 准备 Node 24 与 pnpm 10.33.2。
2. 在 `external/open-design` 执行 `pnpm install`。
3. 执行 `pnpm tools-dev run web`。
4. 打开本地 web URL，检查 `/api/health`、`/api/agents`。
5. 检测本机 Codex/OpenCode/Claude Code 至少一个 agent。
6. 生成一个简单 SaaS landing prototype。
7. 尝试 comment/refine 或 tweaks。
8. 导出 HTML/PDF。
9. 用 handoff prompt 交给 Codex 或 OpenCode 改成 React/Next 项目。
10. 记录启动耗时、生成耗时、失败点、Windows native 问题。

并行做 `open-codesign` 快速验证：

1. `pnpm install`
2. `pnpm dev`
3. 用 BYOK 或本地 provider 生成一个 artifact。
4. 验证 preview/tweaks/comment/export。
5. 看它的 `packages/core/src/tools` 能否拆成我们自己的 kernel 工具层。

## 当前决策建议

先不要手搓 kernel。先把 `nexu-io/open-design` 当作主候选跑通；如果 smoke test 结果可接受，就进入“fork + 减法 + 更好壳子”的路线。

同时保留 `open-codesign` 作为 Plan B。它更适合我们在不满意 `open-design` 复杂度时，快速抽一个轻 kernel 出来。
