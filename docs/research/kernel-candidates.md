# 内核候选调研（历史）

**日期**：2026-06-29  
**结论**：曾建议 fork 大型上游；**项目已改为自研轻量 Rust 内核（`odl`）**。本文仅保留决策背景。

## 当时结论摘要

| 候选 | 评分 | 结论 |
|------|------|------|
| nexu-io/open-design | 9/10 完整度 | 功能全但体量大（Node/Electron/daemon），品牌重叠 |
| OpenCoworkAI/open-codesign | 7.5/10 | MIT、较轻，但外部 CLI adapter 非一等公民 |
| ZSeven-W/openpencil | 6/10 | 适合借鉴 canvas/MCP，非 artifact-first 主内核 |
| llamacoder / open-artifacts | 低 | Artifacts 克隆，不够 Design 工作流 |

## 为何未采纳 fork

1. **产品定位变化**：从「Claude Design 对标」收窄为「轻量本地三模式工具」
2. **ADR 0001**：二进制壳层优先，拒绝重型 Web/daemon 栈
3. **可控性**：Rust 小 workspace，依赖最小，产物纯文件
4. **Windows 体验**：避免 Node 24 + pnpm + Electron 原生依赖负担

## 可借鉴概念（不搬代码）

| 来源 | 借鉴点 |
|------|--------|
| open-design | CLI/MCP adapter 思路、skill 元数据、handoff |
| open-codesign | preview/tweaks 工具清单、质量验证 |
| openpencil | design-as-code 文件模型、MCP 读改设计文件 |

## 本地克隆

调研克隆位于 `external/`（gitignore）。**不要提交上游仓库。**

若需重读原始长文，见 git 历史中的 `docs/research/2026-06-29-open-design-kernel-research.md`。
