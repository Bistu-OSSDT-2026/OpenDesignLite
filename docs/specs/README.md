# 实现规范（Specs）

**产品文档说「做什么」；架构文档说「怎么分层」；本目录说「接口长什么样」。**

每份 spec 就绪后才应实现对应功能。写 spec 时保持：可测试、无歧义、一份真相。

## 规范清单

| Spec | 负责人 | 里程碑 | 状态 |
|------|--------|--------|------|
| [artifact-workspace](artifact-workspace.md) | Product & Kernel Lead | M1 | **待写** |
| [preview](preview.md) | Preview Shell | M1 | **待写** |
| [handoff](handoff.md) | Product & Kernel Lead | M1 | **待写** |
| [cli](cli.md) | Product & Kernel Lead | M1 | **待写** |
| [built-in-skills](built-in-skills.md) | Skills & Templates | M1 | **待写** |
| [smoke-prompts](smoke-prompts.md) | Skills & Templates | M3 | **待写** |
| [mcp](mcp.md) | Product & Kernel Lead | M2 | **待写** |
| [export](export.md) | Export & Packaging | M4 | **待写** |
| [release-checklist](release-checklist.md) | Product & Kernel Lead | M1 末 | **待写** |

## Spec 文档模板

每份 spec 使用以下结构（复制到新文件）：

```markdown
# <名称>

**状态**：草案 | 评审中 | 已定稿  
**里程碑**：M?  
**实现位置**：crates/... 或 skills/...

## 目的

一段话说明规范解决什么问题。

## 范围

- 包含 …
- 不包含 …

## 接口

### 输入 / 输出

（数据结构、文件布局、命令行、JSON schema）

## 行为

### 正常路径

### 错误与退出码

## 示例

（最小可运行示例）

## 测试

（如何验证符合规范）

## 变更记录

| 日期 | 变更 |
|------|------|
```

## 建议编写顺序

```text
1. artifact-workspace  ─┐
2. preview              ├─ M1 并行，但 artifact-workspace 应先定稿
3. handoff              │
4. cli                  ─┘
5. built-in-skills
6. mcp                  ── M2
7. export               ── M4
8. smoke-prompts        ── M3 前
9. release-checklist      ── M1 出口
```

## 占位文件

下方链接文件目前仅为锚点；编写时替换为完整 spec。

### artifact-workspace

→ [artifact-workspace.md](artifact-workspace.md)

### preview

→ [preview.md](preview.md)

### handoff

→ [handoff.md](handoff.md)

### cli

→ [cli.md](cli.md)

### built-in-skills

→ [built-in-skills.md](built-in-skills.md)

### mcp

→ [mcp.md](mcp.md)

### export

→ [export.md](export.md)

### smoke-prompts

→ [smoke-prompts.md](smoke-prompts.md)

### release-checklist

→ [release-checklist.md](release-checklist.md)
