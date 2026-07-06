# Handoff

**状态**：草案  
**里程碑**：M1  
**实现位置**：`crates/od-core`、`skills/*/SKILL.md`

## 目的

定义 `handoff.md` 的结构。handoff 是给外部 Agent 和人类的交接文件，必须解释产物意图、文件结构、设计约束、下一步建议。

## 范围

- 包含通用章节结构。
- 包含三类 artifact 的必填内容。
- 包含 design kernel 说明。
- 包含 agent-specific 备注。
- 不包含 MCP tool schema。

## 文件位置

```text
<artifact>/handoff.md
```

`handoff.md` 是普通 Markdown 文件，可由用户编辑。刷新时不得无提示覆盖用户新增内容；M1 可采用整体重写，但必须在 CLI 输出 warning，M2 应做 section-level 更新。

## 标准结构

```markdown
# Handoff: <title>

## Intent

## Artifact

## Files

## Design Notes

## How To Preview

## Next Steps

## Agent Notes
```

章节规则：

| 章节 | 必填 | 内容 |
|------|------|------|
| `Intent` | 是 | 用户目标、受众、当前状态。 |
| `Artifact` | 是 | kind、主文件、root 路径。 |
| `Files` | 是 | 关键文件列表。 |
| `Design Notes` | 是 | visual brief、token/recipe 使用约束。 |
| `How To Preview` | 是 | `odl preview <dir>` 命令。 |
| `Next Steps` | 是 | 建议后续任务。 |
| `Agent Notes` | 是 | 外部 Agent 操作注意事项。 |

## Design Notes 要求

必须包含：

- 使用 `assets/od-design.css` 或 inline `data-od-design`。
- 使用 `--od-*` token，不引入新的全局 token 前缀。
- 不要求 Tailwind、React、shadcn/ui 或 dev server。
- 如果用户要求目标栈，可作为 adapter 输出，不改 core artifact 约束。

示例：

```markdown
## Design Notes

- Visual brief: `editorial`.
- Styles use `assets/od-design.css` and `--od-*` CSS variables.
- Keep the artifact previewable as plain files; do not add a dev server.
```

## Agent Notes

默认文案：

```markdown
You can edit the files directly. Preserve the artifact layout and keep paths relative to the artifact root. If you add assets, place them under `assets/`. Run `odl preview .` from this directory to inspect changes.
```

Agent-specific 只允许增加提示，不允许改变文件布局规则。

## 测试

- 新建三类 artifact 时生成 handoff。
- handoff 包含 preview 命令。
- handoff 包含 design kernel 约束。
- handoff 中列出的主文件与 manifest 一致。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
