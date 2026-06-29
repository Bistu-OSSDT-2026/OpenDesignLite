pub const MCP_SERVER_NAME: &str = "open-design-lite";

pub fn planned_tools() -> &'static [&'static str] {
    &[
        "odl_create_artifact",
        "odl_preview_artifact",
        "odl_export_artifact",
        "odl_handoff_artifact",
    ]
}
