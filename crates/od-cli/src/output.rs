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

    /// 输出错误。JSON 模式写 stdout 信封，否则写 stderr。
    /// M1 会用 serde_json 生成完整信封（含 path 等字段）；本次为最小占位。
    pub fn error(&self, code: &str, message: &str) {
        if self.json {
            let message = message.replace('\\', "\\\\").replace('"', "\\\"");
            println!("{{\"ok\":false,\"error\":{{\"code\":\"{code}\",\"message\":\"{message}\"}}}}");
        } else {
            eprintln!("error[{code}]: {message}");
        }
    }
}
