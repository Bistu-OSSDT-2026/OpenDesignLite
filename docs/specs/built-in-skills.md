# Built-in Skills

**状态**：草案（2026-07-07 补充 Rust 模型与接入流程）  
**里程碑**：M1  
**实现位置**：`skills/`、`templates/`、`crates/od-core/src/skill.rs`、`crates/od-cli`

## 目的

定义内置技能目录、`SKILL.md` front matter、模板引用规则、Rust 模型、workspace 覆盖、`odl new` 接入流程和质量标准。技能是提示词与模板的文件系统目录，不是 Rust 硬编码逻辑。

## 已采纳决策（本次补充）

- **模板位置**：收进 `skills/<name>/templates/`，skill 自包含；顶层 `templates/` 只留公共片段。
- **`odl new` 接入力度**：M1 保留 fallback，skill 找不到或模板缺失时退回现有 `include_str!` starter；后续稳定后可改为 skill 缺失即报错。
- **front matter 解析器**：M1 手写 line-based，不引入 `serde_yaml`（遵循 AGENTS.md）。

## 技能目录

每个 skill 自包含：`SKILL.md` + 该 skill 专属模板。模板收进 skill 目录，而不是顶层 `templates/`，使新增 skill = 新增一个目录，零 Rust 改动。顶层 `templates/` 仅保留公共片段（如 `od-design.css` starter）。

```text
skills/
  html-page/
    SKILL.md
    templates/
      basic.html
  docs-polish/
    SKILL.md
    templates/
      basic.md
  slides-html/
    SKILL.md
    templates/
      basic.html
templates/
  od-design.css        # 公共 design kernel starter，非任何 skill 专属
```

## Front Matter

最小字段：

```yaml
---
name: html-page
mode: html
description: Create or improve a single-file HTML artifact.
---
```

字段规则：

| 字段 | 必填 | 说明 |
|------|------|------|
| `name` | 是 | 目录名一致，kebab-case。 |
| `mode` | 是 | `html`、`docs`、`slides`。 |
| `description` | 是 | 一句话用途说明。 |
| `template` | 否 | 默认模板路径，相对 skill 目录（如 `templates/basic.html`）。 |
| `visualBrief` | 否 | 默认 brief：`editorial` / `studio` / `workbench`。 |

M1 解析只要求前三个字段。额外字段必须忽略而不是报错。

`template` 路径规则：

- 相对 skill 目录，解析时拼成绝对路径。
- 不允许 `..` 越出 skill 目录。
- 缺失时 `odl new` 退回内置 starter（见下文「`odl new` 接入」fallback）。

## 内置技能映射

| Skill | Mode | Artifact kind | 主文件 | 默认 brief |
|-------|------|---------------|--------|------------|
| `html-page` | `html` | `html` | `index.html` | `editorial` |
| `docs-polish` | `docs` | `docs` | `doc.md` | `editorial` |
| `slides-html` | `slides` | `slides` | `slides.html` | `studio` |

## Rust 模型

实现位置：`crates/od-core/src/skill.rs`。技能 front matter 解析是内核能力，CLI / MCP / shell 必须复用，不得各自推断（见 [boundaries.md](../architecture/boundaries.md)）。

### 两层类型

`SkillFrontMatter`（已存在）：纯解析结果，可对字符串单测，不碰磁盘。

```rust
pub struct SkillFrontMatter {
    pub name: String,
    pub mode: String,
    pub description: String,
    pub template: Option<String>,
    pub visual_brief: Option<String>,
}

impl SkillFrontMatter {
    pub fn parse(source: &str) -> Result<Self>;  // M1 实现，替换现有 todo!()
    pub fn kind(&self) -> Option<ArtifactKind>;   // 已存在
}
```

`Skill`（新增）：解析结果 + 目录位置，能算模板绝对路径。

```rust
pub struct Skill {
    pub front: SkillFrontMatter,
    pub root: PathBuf,            // skill 目录绝对路径
}

impl Skill {
    pub fn from_dir(dir: &Path) -> Result<Self>;            // 读 <root>/SKILL.md 并解析
    pub fn template_path(&self) -> Option<PathBuf>;         // front.template → 绝对路径
    pub fn kind(&self) -> Option<ArtifactKind>;
    pub fn name(&self) -> &str;
}
```

### 发现与查找

```rust
/// 内置 dir + 可选 workspace dir，后者同名覆盖前者。
pub fn discover(builtin: &Path, workspace: Option<&Path>) -> Vec<Skill>;

/// 按 artifact kind 找默认 skill（给 `odl new` 用）。
pub fn for_kind<'a>(skills: &'a [Skill], kind: ArtifactKind) -> Option<&'a Skill>;

/// 按 name 查找。
pub fn find<'a>(skills: &'a [Skill], name: &str) -> Option<&'a Skill>;
```

### 解析器约束

M1 手写 line-based front matter 解析器，不引入 `serde_yaml`（遵循 AGENTS.md「runtime 依赖最小」）：

1. 源必须以 `---\n` 开头，找下一行独立 `---` 作为 front matter 结束。
2. 中间按 `key: value` 拆，trim 两端空白，value 引号可选。
3. `name` / `mode` / `description` 必填，缺失 → `OdError`。
4. `template` / `visual_brief` 可选。
5. 未知 key 忽略，不报错（spec 已规定）。

错误类型：`OdError` 新增 `SkillFrontMatterInvalid(String)` 变体，携带原因（如 `missing field: mode`）。

## Workspace 覆盖

工作区 `<workspace>/skills/` 可覆盖内置 skill（见 [artifact-workspace.md](artifact-workspace.md) 工作区布局）。

规则：

- **发现顺序**：内置 `skills/` → workspace `<ws>/skills/`，后者同名覆盖前者。
- **覆盖粒度**：整个 skill 目录覆盖，不做单文件 merge。符合「目录即单位」原则。
- **解析结果**：同名时 workspace 版本优先，`discover` 返回的 vec 里每个 name 唯一。
- **不报错**：workspace 没有同名 skill 时静默使用内置版。

这让团队能在不改二进制的情况下定制提示词与模板，对 handoff 给外部 agent 场景实用。

## `odl new` 接入

当前 `odl new` 硬编码 `include_str!` 模板（`crates/od-cli/src/commands/new.rs`），完全绕过 `skills/`。本节定义接入流程，把 skill 从「死文档」变成工具消费的配置。

### 数据流

```text
odl new <kind> <dir> [--brief <b>] [--title <t>] [--embed-css] [--force]
  → ArtifactKind::from_slug(kind)
  → skills = discover(BUILTIN_SKILLS_DIR, workspace_skills)
  → skill  = for_kind(skills, kind)
  → template_content:
      skill.template_path() 存在 → fs::read_to_string
      否则                    → fallback starter（现有 include_str!）
  → brief  = --brief > skill.visual_brief > VisualBrief::default_for(kind)
  → 写主文件（template_content）
  → 写 manifest.json（ArtifactManifest，含 design.visualBrief）
  → 写 assets/od-design.css（除非 --embed-css inline）
  → 写 handoff.md
```

### Fallback 规则

skill 找不到、模板路径缺失、或模板读失败时，退回现有 `include_str!` starter，不破坏 M0 行为。fallback 时 CLI 输出 warning，提示用户 skill 未生效。

M1 保留 fallback；后续稳定后可移除，改为 skill 缺失即报错。

### Flag 接入

`cli.rs` 已定义 `--title` / `--brief` / `--embed-css` / `--force`，但 `main.rs` dispatch 当前未传递。实现时需把四项传入 `new::run`。

### 内置 skill 目录定位

`include_str!` 是编译期嵌入，但 skill 发现要运行期读磁盘。M1 用 `env!("CARGO_MANIFEST_DIR")` 拼路径（开发期可用）；发布期嵌入问题留到 M4 打包（候选：`include_dir!` 把 skills/ 嵌进二进制）。

## `odl skill` 命令

新增子命令，让 skill 对用户和外部 agent 可见。M2 的 MCP `skill_list` tool 将镜像同一份 `discover` + 格式化逻辑。

```text
odl skill            # 列出 name | mode | description
odl skill --json     # 输出 JSON 数组
```

M1 只做列表；`skill show <name>`（输出 SKILL.md 正文，供 agent 消费）留到 M2。

## 质量标准

所有技能必须强调：

- 产物是普通文件。
- 默认不启动 dev server。
- 默认不引入 React、Tailwind、shadcn/ui、CDN UI kit。
- HTML / Slides 使用 design kernel token 或静态 CSS。
- Markdown 保持事实准确，避免改写技术含义。
- 输出必须能被 `odl preview` 预览。

## 模板规则

- 模板必须可离线打开。
- HTML 和 slides 模板必须引用或内嵌 `od-design.css`。
- 模板中的路径必须相对 artifact root。
- 模板不得依赖远程字体、远程图片或 CDN 作为默认路径。
- 如果示例需要图片，使用纯 CSS placeholder。

## 测试

每条落成可执行测试，作为 CI 守门，而非文档里的一句话。

| 要求 | 测试类型 | 落点 | 断言 |
|------|----------|------|------|
| 每个 `SKILL.md` front matter 可解析 | `od-core` 单测 | `skill.rs` | `SkillFrontMatter::parse` 对三个内置 SKILL.md 内容解析成功，字段值正确 |
| `mode` 能映射到 artifact kind | `od-core` 单测 | `skill.rs` | `kind()` 对 `html`/`docs`/`slides` 三种 mode 返回对应 `ArtifactKind` |
| 每个 template 可被复制为 artifact 主文件 | 集成测 | `od-cli` | `odl new` 用 skill 模板生成产物，主文件存在且非空 |
| HTML / slides template 无默认 CDN 依赖 | `od-core` 单测 | 扫 `skills/**/templates/*.html` | 断言无 `<script src="https://` 与 `<link href="https://`（允许相对路径） |
| workspace 覆盖生效 | `od-core` 单测 | `discover` 用 tempdir | 同名 skill 时 workspace 版本优先，结果 name 唯一 |
| fallback 保留 | 集成测 | `od-cli` | skill 缺失时 `odl new` 仍生成 starter 主文件，且输出 warning |
| `odl skill` 列表 | 集成测 | `od-cli` | `odl skill` 输出三行，`--json` 输出可被 `serde_json` 解析的数组 |

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-07 | 补 Rust 模型（`Skill` / `discover` / `for_kind`）、解析器约束、workspace 覆盖规则、`odl new` 接入流程与 fallback、`odl skill` 命令；模板收进 skill 目录；测试落成可执行表。 |
