//! od-core：Open Design Lite 内核。
//!
//! 拥有 artifact/workspace 路径、manifest schema、design kernel 语义、skill 模型、
//! handoff 与稳定错误类型。入口层（od-cli / od-mcp / od-preview）必须复用这里的规则，
//! 不得各自推断路径或重复实现 manifest 解析。
//!
//! Spec: docs/specs/{artifact-workspace,design-kernel,built-in-skills,handoff}.md

pub mod artifact;
pub mod design;
pub mod error;
pub mod handoff;
pub mod manifest;
pub mod paths;
pub mod skill;
pub mod workspace;

pub use artifact::{Artifact, ArtifactKind};
pub use error::{OdError, Result};
pub use workspace::workspace_manifest_path;
