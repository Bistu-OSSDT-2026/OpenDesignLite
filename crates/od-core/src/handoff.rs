//! handoff.md 章节模型与渲染签名。
//!
//! Spec: docs/specs/handoff.md

use crate::artifact::Artifact;

/// handoff.md 标准章节顺序（集成契约）。
pub const SECTIONS: [&str; 7] = [
    "Intent",
    "Artifact",
    "Files",
    "Design Notes",
    "How To Preview",
    "Next Steps",
    "Agent Notes",
];

/// 目标 Agent，影响 `Agent Notes` 的附注文案（不改变文件布局规则）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffAgent {
    Generic,
    OpenCode,
    ClaudeCode,
    Codex,
}

impl HandoffAgent {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "generic" => Some(Self::Generic),
            "opencode" => Some(Self::OpenCode),
            "claude-code" => Some(Self::ClaudeCode),
            "codex" => Some(Self::Codex),
            _ => None,
        }
    }
}

/// 渲染完整 handoff.md。M1 按标准章节实现。
pub fn render(_artifact: &Artifact, _agent: HandoffAgent) -> String {
    todo!("M1: render standard handoff sections; see docs/specs/handoff.md")
}

/// M0 兼容：最小占位 handoff，保持 `odl new` 现有行为直到 M1 完整渲染上线。
pub fn minimal_stub() -> &'static str {
    "# Handoff\n\nDescribe intent, constraints, and next agent steps here.\n"
}
