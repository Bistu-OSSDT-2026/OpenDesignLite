# Built-in Skills

**状态**：草案  
**里程碑**：M1  
**实现位置**：`skills/`、`templates/`

## 目的

定义内置技能目录、`SKILL.md` front matter、模板引用规则和质量标准。技能是提示词与模板的文件系统目录，不是 Rust 硬编码逻辑。

## 技能目录

```text
skills/
  html-page/
    SKILL.md
  docs-polish/
    SKILL.md
  slides-html/
    SKILL.md
templates/
  html-page/
    basic.html
  slides/
    basic.html
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
| `template` | 否 | 默认模板路径。 |
| `visualBrief` | 否 | 默认 brief。 |

M1 解析只要求前三个字段。额外字段必须忽略而不是报错。

## 内置技能映射

| Skill | Mode | Artifact kind | 主文件 | 默认 brief |
|-------|------|---------------|--------|------------|
| `html-page` | `html` | `html` | `index.html` | `editorial` |
| `docs-polish` | `docs` | `docs` | `doc.md` | `editorial` |
| `slides-html` | `slides` | `slides` | `slides.html` | `studio` |

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

- 每个 `SKILL.md` front matter 可解析。
- `mode` 能映射到 artifact kind。
- 每个 template 可被复制为 artifact 主文件。
- HTML / slides template 不包含默认 CDN 依赖。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
