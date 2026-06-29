# 架构

## 决策

构建轻量级本地内核与原生二进制壳层。以 CLI/MCP 作为稳定的集成契约。将编辑器插件视为可选适配层。

## 运行时层次

```text
原生壳层 / CLI / MCP / 扩展
              |
              v
        od-core 内核
              |
   +----------+----------+
   |          |          |
技能       产物      预览/导出
   |          |          |
磁盘上的纯文件    系统 WebView / Markdown 渲染器
```

## 为何优先二进制壳层

用户体验目标是快速与本地化。二进制壳层可以：

- 避免启动 Web 应用栈
- 提供统一的预览实现
- 可从任意编辑器或终端启动
- 使内核独立于 Codex/Cursor/Zed 等 API

## 为何不优先插件

插件优先的产品会立即碎片化：

- Cursor、VS Code、Zed、Codex 和 Claude Code 的扩展接口各不相同。
- 各编辑器的预览面板行为不一致。
- 发布与权限模型不同。
- 核心产品会被困在我们最先构建的插件之中。

因此，插件应调用 `odl` 或 MCP，并展示相同的产物。

## 产物模型

产物是一个文件夹：

```text
my-artifact/
  index.html | doc.md | slides.html
  assets/
  manifest.json
  handoff.md
```

一切内容应保持人类可读、Agent 可读。

## 技能模型

技能是一个文件夹：

```text
skills/html-page/
  SKILL.md
  templates/
```

第一版只需可读的 Markdown 说明，以及可选模板。在内置技能做到优秀之前，不需要注册表或市场。

## Agent 集成

MVP 支持三个层次：

- 交接文件：为外部 Agent 生成提示词与上下文。
- CLI：`odl new`、`odl preview`、`odl export`、`odl handoff`。
- MCP：暴露产物创建/预览/导出/交接工具。

本地工作流验证通过后，可再增加直接模型调用。

## 预览

预览是本地、基于文件的：

- HTML/幻灯片：系统 WebView。
- Markdown：原生渲染器，或在 WebView 中展示转换后的静态 HTML。
- 未来 PDF：从 WebView 打印/导出。

生成的产物不需要开发服务器。
