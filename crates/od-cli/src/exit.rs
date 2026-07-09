//! 退出码映射（集成契约）。
//!
//! Spec: docs/specs/cli.md（退出码表）

use crate::error::CliError;
use od_core::OdError;
use od_preview::PreviewError;

// 完整退出码契约（cli.md）。部分码在 M1/M4 命令接线后才被引用，先保留。
#[allow(dead_code)]
pub const OK: i32 = 0;
pub const GENERAL: i32 = 1;
pub const USAGE: i32 = 2;
pub const INVALID_ARTIFACT: i32 = 3;
pub const PREVIEW_FAILED: i32 = 4;
#[allow(dead_code)]
pub const EXPORT_FAILED: i32 = 5;
pub const NOT_IMPLEMENTED: i32 = 10;

/// 由 CLI 错误推断退出码。clap 自身的参数错误由 clap 以退出码 2 处理。
pub fn code_for(err: &CliError) -> i32 {
    match err {
        CliError::Core(e) => code_for_core(e),
        CliError::Preview(e) => code_for_preview(e),
    }
}

fn code_for_core(err: &OdError) -> i32 {
    match err {
        OdError::ArtifactKindUnknown(_) => USAGE,
        OdError::WorkspaceNotFound(_)
        | OdError::PrimaryFileMissing(_)
        | OdError::ManifestInvalid { .. }
        | OdError::AlreadyExists(_)
        | OdError::PathEscape(_)
        | OdError::SkillFrontMatterInvalid(_) => INVALID_ARTIFACT,
        OdError::NotImplemented(_) => NOT_IMPLEMENTED,
        OdError::Io(_) => GENERAL,
    }
}

fn code_for_preview(err: &PreviewError) -> i32 {
    match err {
        PreviewError::ArtifactNotFound(_) | PreviewError::PrimaryFileMissing(_) => {
            INVALID_ARTIFACT
        }
        PreviewError::RenderFailed(_)
        | PreviewError::WebviewFailed(_)
        | PreviewError::WatchFailed(_)
        | PreviewError::FallbackFailed(_) => PREVIEW_FAILED,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_kind_is_usage_error() {
        let err = CliError::Core(OdError::ArtifactKindUnknown("pdf".into()));
        assert_eq!(code_for(&err), USAGE);
    }

    #[test]
    fn not_implemented_maps_to_10() {
        let err = CliError::Core(OdError::NotImplemented("x"));
        assert_eq!(code_for(&err), NOT_IMPLEMENTED);
    }

    #[test]
    fn preview_failure_maps_to_4() {
        let err = CliError::Preview(PreviewError::WebviewFailed("boom".into()));
        assert_eq!(code_for(&err), PREVIEW_FAILED);
    }
}
