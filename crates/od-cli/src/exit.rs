//! 退出码映射（集成契约）。
//!
//! Spec: docs/specs/cli.md（退出码表）

use od_core::OdError;

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

/// 由内核错误推断退出码。clap 自身的参数错误由 clap 以退出码 2 处理。
pub fn code_for(err: &OdError) -> i32 {
    match err {
        OdError::ArtifactKindUnknown(_) => USAGE,
        OdError::SkillNotFound(_) => USAGE,
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

/// 由预览错误推断退出码。
pub fn code_for_preview(err: &od_preview::PreviewError) -> i32 {
    match err {
        od_preview::PreviewError::ArtifactNotFound(_)
        | od_preview::PreviewError::PrimaryFileMissing(_) => INVALID_ARTIFACT,
        od_preview::PreviewError::RenderFailed(_)
        | od_preview::PreviewError::WebviewFailed(_)
        | od_preview::PreviewError::WatchFailed(_) => PREVIEW_FAILED,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_kind_is_usage_error() {
        let err = OdError::ArtifactKindUnknown("pdf".into());
        assert_eq!(code_for(&err), USAGE);
    }

    #[test]
    fn not_implemented_maps_to_10() {
        assert_eq!(code_for(&OdError::NotImplemented("x")), NOT_IMPLEMENTED);
    }

    #[test]
    fn skill_front_matter_invalid_is_invalid_artifact() {
        let err = OdError::SkillFrontMatterInvalid("missing field: mode".into());
        assert_eq!(code_for(&err), INVALID_ARTIFACT);
    }

    #[test]
    fn skill_not_found_is_usage_error() {
        let err = OdError::SkillNotFound("nope".into());
        assert_eq!(code_for(&err), USAGE);
    }

    #[test]
    fn preview_render_error_is_preview_failed() {
        let err = od_preview::PreviewError::RenderFailed("x".into());
        assert_eq!(code_for_preview(&err), PREVIEW_FAILED);
    }
}
