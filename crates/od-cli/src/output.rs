//! 用户输出：human 文本与 `--json` 信封。
//! 普通信息→stdout；warning/error→stderr；`--json` 时 stdout 只输出 JSON。
//!
//! Spec: docs/specs/cli.md（JSON 输出 / 日志规则）

use serde_json::Value;

pub struct Reporter {
    pub json: bool,
    pub quiet: bool,
}

impl Reporter {
    pub fn info(&self, msg: &str) {
        if !self.quiet && !self.json {
            println!("{msg}");
        }
    }

    /// 输出成功信封（`--json` 模式写 stdout，否则忽略）。
    pub fn success_artifact(&self, payload: &Value) {
        if self.json {
            let envelope = serde_json::json!({ "ok": true, "artifact": payload });
            println!("{}", envelope);
        }
    }

    /// 输出 warning 到 stderr（`--json` 模式仍写 stderr，不打扰 stdout 信封）。
    pub fn warning(&self, msg: &str) {
        if !self.quiet {
            eprintln!("warning: {msg}");
        }
    }

    /// 输出错误。JSON 模式写 stdout 信封，否则写 stderr。
    pub fn error(&self, code: &str, message: &str) {
        if self.json {
            let envelope = serde_json::json!({
                "ok": false,
                "error": { "code": code, "message": message }
            });
            println!("{}", envelope);
        } else {
            eprintln!("error[{code}]: {message}");
        }
    }
}
