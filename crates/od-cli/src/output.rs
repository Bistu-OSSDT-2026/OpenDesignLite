//! 用户输出：human 文本与 `--json` 信封。
//! 普通信息→stdout；warning/error→stderr；`--json` 时 stdout 只输出 JSON。
//!
//! Spec: docs/specs/cli.md（JSON 输出 / 日志规则）

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

    pub fn warn(&self, msg: &str) {
        if !self.quiet && !self.json {
            eprintln!("warning: {msg}");
        }
    }

    pub fn json_value(&self, value: serde_json::Value) {
        if self.json {
            println!("{value}");
        }
    }

    /// 输出错误。JSON 模式写 stdout 信封，否则写 stderr。
    pub fn error(&self, code: &str, message: &str) {
        if self.json {
            println!(
                "{}",
                serde_json::json!({
                    "ok": false,
                    "error": {
                        "code": code,
                        "message": message,
                    }
                })
            );
        } else {
            eprintln!("error[{code}]: {message}");
        }
    }
}
