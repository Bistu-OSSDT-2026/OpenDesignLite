use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewSurface {
    NativeShell,
    ExternalBrowser,
    EditorPanel,
}

pub fn can_preview(path: impl AsRef<Path>) -> bool {
    matches!(
        path.as_ref().extension().and_then(|ext| ext.to_str()),
        Some("html" | "htm" | "md")
    )
}
