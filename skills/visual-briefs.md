# Visual Briefs — 美术方向词汇表

这是 `SKILL.md` front matter 里 `visualBrief` 字段的权威定义。三个 brief 是给生成 agent 的**美术指导锚点**：一句话告诉它"往哪个视觉方向走"，再用本文件把那一句话展开成可执行的约束。

## 它在产物里长什么样

- `SKILL.md` 写 `visualBrief: editorial` → `odl new` 把它写进 `manifest.json` 的 `design.visualBrief` 和 `handoff.md` 的 `- visual brief:` 行。
- `--brief` 命令行参数可覆盖 skill 的默认值；未知值时回退到该 artifact kind 的默认 brief（`html`/`docs → editorial`，`slides → studio`）。
- 产物本身是文件；本文件是供人和 agent 读的美术规范。

设计 token 与 class 全部锚定 `templates/od-design.css`（暖纸色 `--od-bg-canvas` #f7f3ed、陶土 accent `--od-accent-solid` #8f5b3f 等，含完整暗色主题），不引用任何该文件之外的变量。

---

## `editorial` — 暖纸编辑风

**一句话**：温暖纸感的杂志页，靠排版和留白说话，装饰极度克制。

- **画布**：页面用 `--od-bg-canvas`（#f7f3ed，暖纸），卡片/强调块用 `--od-bg-surface` / `--od-bg-elevated`。不要往灰白冷调走——暖色是这个方向的信号。
- **色彩**：大面积是 ink-on-paper。正文字 `--od-text-secondary`，标题字 `--od-text-primary`；陶土色 `--od-accent-*` **只**用于链接、单个主 CTA、小面积点缀。不用 accent 做大面积铺色。三个颜色内是完成的；七个颜色通常不是。
- **字体**：展示标题用 `--od-font-serif`（Georgia），正文用 `--od-font-sans`。靠**字号与字重**拉开对比，不打色彩差。
- **行宽**：正文列限制在 `.od-container` 880px 以内，可读优先；不要把正文铺满整个 viewport。
- **分隔**：主要用垂直节奏（`--od-space-8` ~ `--od-space-10`）和 `--od-border-subtle` 细线分隔区块。避免给每块都套卡片——卡片是偶尔强调，不是默认容器。
- **阴影**：尽量不用；用阴影 = 显得重，与编辑风相悖。需要轻浮起时只用 `--od-shadow-xs`。
- **动效**：克制。只在交互需要时加，duration 用 `--od-duration-fast`/`-base`，遵循 `prefers-reduced-motion`。
- **适用**：文档、落地页、叙事/营销页、文章页。对应 `.od-hero`、`.od-container`、`.od-stack`、`.od-card`（节制使用）、`.od-doc`（Markdown 预览）。

---

## `studio` — 安静工作室风

**一句话**：画廊墙 / 设计评图板，每个页面是单个想法的安静舞台，留白是设计本身。

- **画布**：用最安静的 `--od-bg-elevated` 作 deck/页面背景；slide 之间用 `--od-border-subtle` 细线分隔全屏区块，不要给每张 slide 套重框带阴影。
- **密度**：低。一页一个焦点。标题大，配最多两行短支撑文字。一页 5 个 bullet 通常是失败——拆分。
- **色彩**：ink-on-paper 为主，`--od-accent-*` 作**记号式**点缀——只在标题页和章节分隔页出现，不给每张正文页染色。
- **视觉优先**：能用一个焦点视觉（`.od-frame` 占位、大图示、图表）就别用文字墙。占位图用 `--od-bg-surface` 实色块、两色 `linear-gradient`、带标签 `.od-frame`——一律不用远程 URL。
- **字体**：标题靠字号站住舞台（`--od-text-4xl`/`-2xl`），正文 `--od-text-base`/`-lg`。
- **画框一致**：每张 slide 同一 16:9 内框（`.od-slide__inner` 已含居中与比例）。变化的是**内容**不是画布几何——不一致的框等于业余。
- **动效**：默认零动画。键盘翻页是唯一交互。想要 "build" 感就拆成更多页，不要做逐项出现。
- **适用**：幻灯片、作品集、设计展示。对应 `.od-slide-deck`、`.od-slide`、`.od-slide__inner`、`.od-frame`、`.od-hero`（标题页）。

---

## `workbench` — 工作台 / 仪表板风

**一句话**：清晰层级 + 紧凑控件 + 低对比背景的工具界面，信息密度优先，不堆装饰。

- **画布**：低对比、偏中性的工具背景。仍用 `--od-bg-canvas` 作页面底，但工具内面板/卡片密集排布时多用 `--od-bg-surface`，分隔靠 `--od-border-default`（比 `subtle` 稍重一档）让层级读得出来。
- **密度**：高，但要分组。用 `.od-dashboard` 或 `.od-grid` 把指标/控件网格化；`--od-space-4`~`-6` 的小间距让多个面板紧凑共存，不像编辑风那样留大透气块。
- **层级**：信息靠**结构**而不是色彩分层——表头、卡片标题用 `--od-text-primary`，指标数值小字标签用 `--od-text-secondary`，元信息/占位用 `--od-text-muted`。三层分明，读者一眼分得清主次。
- **色彩**：accent 严格保留给**状态与可操作项**——主按钮用 `.od-button[data-variant="primary"]`，状态用 `.od-badge[data-tone="success|warning|danger|accent"]`。不要用 accent 给大区块铺底；背景始终中性，颜色只点信号。
- **紧凑控件**：所有可交互目标高度 ≥ 40px（`.od-button` `.od-input` 已达标），阻隔开关用 `.od-table` / `.od-badge` / `.od-card`。控件之间用 `.od-cluster` 收一圈工具栏。
- **字体**：正文 `--od-font-sans` 为主，等宽 `--od-font-mono` 用于数值/代码/标识符/路径——仪表板里 mono 字体让数据稳定对齐。
- **阴影/边框**：卡片默认 `--od-shadow-xs` 轻浮起即可；不要用 `--od-shadow-md` 把面板抬得太高，工具界面靠边框分块更清爽。
- **动效**：只给**状态变化**反馈用（数据更新、hover），不做装饰性入场动画。duration 用 `--od-duration-fast`。
- **适用**：工具页、dashboard、artifact workspace、指标卡网格。对应 `.od-dashboard`、`.od-grid`、`.od-card`、`.od-table`、`.od-badge`、`.od-input`、`.od-button`、`.od-cluster`、`.od-empty`。

---

## 选择与覆盖规则

| 信号来源 | 优先级 |
|----------|--------|
| `odl new --brief <b>` | 最高 |
| skill 的 `visualBrief` | 中（skill 的默认值） |
| `VisualBrief::default_for(kind)` | 最低（html/docs → editorial，slides → studio） |

`workbench` 没有内置 skill 用它作为默认——它预留给未来的 dashboard / 工具类 skill，以及 workspace 自定义 skill 调用。引用本文件里未定义的 brief 名（如 `instrument`、`gallery`）不被接受：M1 仅 `editorial` / `studio` / `workbench` 三个。