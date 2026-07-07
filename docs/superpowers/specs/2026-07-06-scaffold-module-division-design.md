# 脚手架与模块划分设计

**日期**：2026-07-06
**范围**：M0 → M1 过渡的**模块划分**（不实现 M1 完整逻辑）
**依据**：`docs/product/prd.md`、`docs/architecture/{overview,boundaries}.md`、`docs/specs/*`

## 目标

把当前 M0 的最小占位代码，重构为**按 spec 划分好模块边界**的编译骨架，为 M1 实现做准备。四个 crate 全覆盖。

## 决策（已与用户确认）

| 项 | 决定 |
|----|------|
| 深度 | 仅模块/目录划分，不实现 M1 完整逻辑 |
| 覆盖 | 全部四个 crate（od-mcp 保持 M2 薄骨架） |
| 依赖 | 允许轻量库：`serde`/`serde_json`/`thiserror`/`clap`；**不接** `wry`/`comrak`/`notify`/`rmcp`/`tokio` |
| 现有逻辑 | `init`/`new`/`preview` 移入新模块，行为不变、仍可运行 |

## 骨架约定

- 每个模块头部 doc-comment 标注**所属 spec**。
- 函数体：能保持现有行为的用最小实现；M1/M2 逻辑用 `todo!()` 或返回 `not_implemented` 错误。
- 未接入的重型依赖（webview/markdown/watch/mcp）只留**签名**与错误类型。
- 每个 crate 内建 `.log/work-log.md`（≤300 字工作日志，符合全局规范；该目录已在 preview/export 的忽略列表内）。
- `cargo build` / `cargo test` 必须通过。

## 模块树

### od-core（内核）
```
error.rs      OdError + 稳定 code（thiserror）        [artifact-workspace/cli]
paths.rs      相对路径归一化为 '/'、path_escape 守卫   [artifact-workspace]
artifact.rs   Artifact / ArtifactKind / 检测顺序        [artifact-workspace]
manifest.rs   Artifact/Workspace/DesignMeta（serde）    [artifact-workspace]
workspace.rs  Workspace 布局                            [artifact-workspace]
design/mod.rs KERNEL_VERSION、资产名                    [design-kernel]
design/brief.rs   VisualBrief{Editorial,Studio,Workbench} [design-kernel]
design/tokens.rs  --od-* / od-design.css 命名常量       [design-kernel]
skill.rs      Skill / SkillFrontMatter、mode↔kind        [built-in-skills]
handoff.rs    Handoff 章节模型 + render() 签名           [handoff]
```

### od-cli（`odl`）
```
cli.rs        clap 派生：Command 枚举 + 全局 flag        [cli]
output.rs     human vs --json 信封 {ok,artifact|error}  [cli]
exit.rs       退出码映射 0/1/2/3/4/5/10                  [cli]
commands/init.rs      现有逻辑移入，可用                 [cli]
commands/new.rs       现有逻辑移入，可用                 [cli]
commands/preview.rs   现有占位移入，委托 od-preview     [cli/preview]
commands/handoff.rs   stub → not_implemented            [handoff]
commands/export.rs    stub → 退出码 10                  [export]
```

### od-preview（预览边界）
```
lib.rs        PreviewOptions + preview() 签名           [preview]
detect.rs     主文件检测（委托 od-core 顺序）           [preview]
render/markdown.rs  comrak→ammonia→minijinja 签名        [preview]
webview.rs    wry 生命周期 签名                          [preview]
watch.rs      notify + debounce 签名                     [preview]
error_page.rs 内联错误 HTML + PreviewError code          [preview]
fallback.rs   外部浏览器 fallback                        [preview]
```

### od-mcp（MCP 工具面，M2 薄骨架）
```
lib.rs        server facade 签名                         [mcp]
error.rs      code 映射 invalid_args/artifact_not_found… [mcp]
tools/{create,preview,handoff,export}.rs  工具签名       [mcp]
```

工具名对齐 spec：`artifact_create` / `artifact_preview` / `artifact_handoff` / `artifact_export`
（修正现有 `planned_tools()` 中 `odl_*_artifact` 的旧命名）。

## 不在本次范围

- M1 完整逻辑（真实 manifest 写入 spec schema、design kernel CSS 生成、真实预览/watch）。
- `apps/shell`、`apps/extensions`（非 crate，保持 README 占位）。
- `skills/`、`templates/`（目录形态已符合 built-in-skills spec）。

## 依赖方向（不变）

`od-cli` / `od-mcp` / `od-preview` → `od-core`。`od-core` 不反向依赖任何入口层，不引入 UI runtime。

## 验收

- `cargo build` 全 workspace 通过。
- `cargo test` 通过（含现有/新增最小单测）。
- `odl init` / `odl new html <dir>` / `odl preview <dir>` 行为与 M0 一致。
- 每个 crate 有 `.log/work-log.md`。
