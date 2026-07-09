//! 内置技能模型：`SKILL.md` front matter 与发现。技能是文件系统目录，不是硬编码逻辑。
//!
//! Spec: docs/specs/built-in-skills.md

use crate::artifact::ArtifactKind;
use crate::error::{OdError, Result};
use crate::paths::ensure_within;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// `SKILL.md` 解析结果：front matter + 正文 body（front matter 结束 `---` 之后的内容）。
///
/// M1 只要求 name / mode / description，额外字段忽略。body 是给 agent 读的提示词正文，
/// 通过 `odl skill show <name>` 消费（spec built-in-skills.md「`odl skill` 命令」M2）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontMatter {
    pub name: String,
    /// `html` | `docs` | `slides`。
    pub mode: String,
    pub description: String,
    pub template: Option<String>,
    pub visual_brief: Option<String>,
    /// front matter 结束 `---` 之后的正文（已 trim，无 front matter 时为空）。
    pub body: String,
}

impl SkillFrontMatter {
    /// mode → artifact kind 映射。
    pub fn kind(&self) -> Option<ArtifactKind> {
        match self.mode.as_str() {
            "html" => Some(ArtifactKind::Html),
            "docs" => Some(ArtifactKind::Markdown),
            "slides" => Some(ArtifactKind::Slides),
            _ => None,
        }
    }

    /// 解析 `SKILL.md` 顶部 YAML front matter，并保留结束 `---` 之后的正文 body。
    ///
    /// 手写 line-based 解析器（不引入 serde_yaml，遵循 AGENTS.md）：
    /// 1. 源须以 `---\n` 开头，找下一行独立 `---` 作为 front matter 结束。
    /// 2. 中间按 `key: value` 拆，trim 两端空白，value 引号可选。
    /// 3. `name` / `mode` / `description` 必填，缺失 → `SkillFrontMatterInvalid`。
    /// 4. `template` / `visual_brief` 可选。
    /// 5. 未知 key 忽略，不报错。
    /// 6. 结束 `---` 之后的内容作为 `body`（trim 首尾空白）返回，供 agent 消费。
    pub fn parse(source: &str) -> Result<Self> {
        let mut lines = source.lines();
        let first = lines.next();
        if first.map(str::trim) != Some("---") {
            return Err(OdError::SkillFrontMatterInvalid(
                "front matter must start with `---`".into(),
            ));
        }

        // 收集到下一个独占一行的 `---`；之后的行收入 body。
        let mut raw: Vec<&str> = Vec::new();
        let mut body_lines: Vec<&str> = Vec::new();
        let mut in_front = true;
        for line in lines {
            if in_front {
                if line.trim() == "---" {
                    in_front = false;
                    continue;
                }
                raw.push(line);
            } else {
                body_lines.push(line);
            }
        }
        // 若上面循环因耗尽而退出（没有结束 `---`），raw 仍可用，但视为缺结束标记。
        // 这里只在根本没有任何 front matter 行时判定；end marker 缺失不强制报错以保持宽松。

        let mut name = None;
        let mut mode = None;
        let mut description = None;
        let mut template = None;
        let mut visual_brief = None;

        for line in raw {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = match line.split_once(':') {
                Some((k, v)) => (k.trim(), v.trim()),
                None => continue, // 非 `key: value` 行忽略
            };
            let value = strip_quotes(value);
            match key {
                "name" => name = Some(value.to_string()),
                "mode" => mode = Some(value.to_string()),
                "description" => description = Some(value.to_string()),
                "template" => template = Some(value.to_string()),
                "visualBrief" | "visual_brief" => visual_brief = Some(value.to_string()),
                _ => {} // 未知 key 忽略
            }
        }

        let name =
            name.ok_or_else(|| OdError::SkillFrontMatterInvalid("missing field: name".into()))?;
        let mode =
            mode.ok_or_else(|| OdError::SkillFrontMatterInvalid("missing field: mode".into()))?;
        let description = description
            .ok_or_else(|| OdError::SkillFrontMatterInvalid("missing field: description".into()))?;

        Ok(Self {
            name,
            mode,
            description,
            template,
            visual_brief,
            body: body_lines.join("\n").trim().to_string(),
        })
    }
}

/// 去掉 value 两端的可选引号（`"..."` 或 `'...'`）。
fn strip_quotes(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && (bytes[0] == b'"' || bytes[0] == b'\'')
        && bytes[0] == bytes[bytes.len() - 1]
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

/// 解析结果 + 目录位置，能算模板绝对路径。
#[derive(Debug, Clone)]
pub struct Skill {
    pub front: SkillFrontMatter,
    /// skill 目录绝对路径。
    pub root: PathBuf,
}

impl Skill {
    /// 读 `<dir>/SKILL.md` 并解析。
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let skill_md = dir.join("SKILL.md");
        let source = fs::read_to_string(&skill_md).map_err(|e| {
            OdError::SkillFrontMatterInvalid(format!("cannot read {}: {e}", skill_md.display()))
        })?;
        let front = SkillFrontMatter::parse(&source)?;
        Ok(Self {
            front,
            root: dir.to_path_buf(),
        })
    }

    /// `front.template` → 相对 root 的绝对路径；拒绝 `..` 越出 skill 目录。
    pub fn template_path(&self) -> Option<PathBuf> {
        let rel = self.front.template.as_ref()?;
        let rel_path = Path::new(rel);
        // 拒绝 `..` 越界。
        ensure_within(&self.root, rel_path).ok()?;
        Some(self.root.join(rel_path))
    }

    pub fn kind(&self) -> Option<ArtifactKind> {
        self.front.kind()
    }

    pub fn name(&self) -> &str {
        &self.front.name
    }

    /// front matter 之后的 `SKILL.md` 正文，给 agent 读的提示词本体。
    pub fn body(&self) -> &str {
        &self.front.body
    }
}

/// 内置 dir + 可选 workspace dir，后者同名覆盖前者（整目录覆盖，非单文件 merge）。
///
/// Spec: docs/specs/built-in-skills.md（Workspace 覆盖）
pub fn discover(builtin: &Path, workspace: Option<&Path>) -> Vec<Skill> {
    let mut by_name: BTreeMap<String, Skill> = BTreeMap::new();

    // 先内置，后 workspace 覆盖。
    for dir in [Some(builtin), workspace].into_iter().flatten() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                if let Ok(skill) = Skill::from_dir(&path) {
                    by_name.insert(skill.name().to_string(), skill);
                }
            }
        }
    }

    by_name.into_values().collect()
}

/// 按 artifact kind 找默认 skill（给 `odl new` 用）。
pub fn for_kind(skills: &[Skill], kind: ArtifactKind) -> Option<&Skill> {
    skills.iter().find(|s| s.kind() == Some(kind))
}

/// 按 name 查找。
pub fn find<'a>(skills: &'a [Skill], name: &str) -> Option<&'a Skill> {
    skills.iter().find(|s| s.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const HTML_SKILL: &str = "---\nname: html-page\nmode: html\ndescription: Create HTML.\ntemplate: templates/basic.html\nvisualBrief: editorial\n---\n\nBody.\n";

    #[test]
    fn parse_builtin_html_skill() {
        let fm = SkillFrontMatter::parse(HTML_SKILL).unwrap();
        assert_eq!(fm.name, "html-page");
        assert_eq!(fm.mode, "html");
        assert_eq!(fm.description, "Create HTML.");
        assert_eq!(fm.template.as_deref(), Some("templates/basic.html"));
        assert_eq!(fm.visual_brief.as_deref(), Some("editorial"));
        assert_eq!(fm.kind(), Some(ArtifactKind::Html));
    }

    #[test]
    fn parse_three_modes_map_to_kinds() {
        for (mode, kind) in [
            ("html", ArtifactKind::Html),
            ("docs", ArtifactKind::Markdown),
            ("slides", ArtifactKind::Slides),
        ] {
            let src = format!("---\nname: x\nmode: {mode}\ndescription: d.\n---\n");
            let fm = SkillFrontMatter::parse(&src).unwrap();
            assert_eq!(fm.kind(), Some(kind));
        }
    }

    #[test]
    fn parse_quoted_values() {
        let src = "---\nname: \"x\"\nmode: 'html'\ndescription: \"d\"\n---\n";
        let fm = SkillFrontMatter::parse(src).unwrap();
        assert_eq!(fm.name, "x");
        assert_eq!(fm.mode, "html");
        assert_eq!(fm.description, "d");
    }

    #[test]
    fn parse_missing_field_errors() {
        let src = "---\nname: x\ndescription: d.\n---\n";
        let err = SkillFrontMatter::parse(src).unwrap_err();
        assert!(err.to_string().contains("mode"));
        assert_eq!(err.code(), "skill_front_matter_invalid");
    }

    #[test]
    fn parse_missing_start_marker_errors() {
        let src = "name: x\nmode: html\n";
        assert!(SkillFrontMatter::parse(src).is_err());
    }

    #[test]
    fn parse_unknown_key_ignored() {
        let src = "---\nname: x\nmode: html\ndescription: d.\nweird: value\n---\n";
        let fm = SkillFrontMatter::parse(src).unwrap();
        assert_eq!(fm.name, "x");
    }

    #[test]
    fn template_path_resolves_and_rejects_escape() {
        let tmp = std::env::temp_dir().join("od-skill-test-escape");
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("SKILL.md"), HTML_SKILL).unwrap();

        let skill = Skill::from_dir(&tmp).unwrap();
        assert_eq!(
            skill.template_path().as_deref(),
            Some(tmp.join("templates/basic.html").as_path())
        );

        // 越界模板路径被拒绝。
        let evil = "---\nname: evil\nmode: html\ndescription: d.\ntemplate: ../escape.html\n---\n";
        fs::write(tmp.join("SKILL.md"), evil).unwrap();
        let evil_skill = Skill::from_dir(&tmp).unwrap();
        assert_eq!(evil_skill.template_path(), None);

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn discover_workspace_overrides_builtin() {
        let builtin = std::env::temp_dir().join("od-skill-builtin");
        let workspace = std::env::temp_dir().join("od-skill-ws");
        let _ = fs::remove_dir_all(&builtin);
        let _ = fs::remove_dir_all(&workspace);

        fs::create_dir_all(builtin.join("html-page")).unwrap();
        fs::write(
            builtin.join("html-page").join("SKILL.md"),
            "---\nname: html-page\nmode: html\ndescription: builtin.\n---\n",
        )
        .unwrap();

        fs::create_dir_all(workspace.join("html-page")).unwrap();
        fs::write(
            workspace.join("html-page").join("SKILL.md"),
            "---\nname: html-page\nmode: html\ndescription: overridden.\n---\n",
        )
        .unwrap();

        let skills = discover(&builtin, Some(&workspace));
        // 同名唯一，workspace 版本优先。
        assert_eq!(skills.iter().filter(|s| s.name() == "html-page").count(), 1);
        let s = find(&skills, "html-page").unwrap();
        assert_eq!(s.front.description, "overridden.");

        // 不传 workspace 时只有内置。
        let only_builtin = discover(&builtin, None);
        let b = find(&only_builtin, "html-page").unwrap();
        assert_eq!(b.front.description, "builtin.");

        fs::remove_dir_all(&builtin).ok();
        fs::remove_dir_all(&workspace).ok();
    }

    #[test]
    fn for_kind_finds_matching_skill() {
        let tmp = std::env::temp_dir().join("od-skill-forkind");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("html-page")).unwrap();
        fs::write(
            tmp.join("html-page").join("SKILL.md"),
            "---\nname: html-page\nmode: html\ndescription: d.\n---\n",
        )
        .unwrap();

        let skills = discover(&tmp, None);
        assert!(for_kind(&skills, ArtifactKind::Html).is_some());
        assert!(for_kind(&skills, ArtifactKind::Slides).is_none());

        fs::remove_dir_all(&tmp).ok();
    }

    const HTML_SKILL_WITH_BODY: &str =
        "---\nname: html-page\nmode: html\ndescription: Create HTML.\n---\n\n# html-page\n\nBody line 1.\nBody line 2.\n";

    #[test]
    fn parse_body_after_front_matter() {
        let fm = SkillFrontMatter::parse(HTML_SKILL_WITH_BODY).unwrap();
        assert_eq!(fm.name, "html-page");
        assert_eq!(
            fm.body,
            "# html-page\n\nBody line 1.\nBody line 2."
        );
    }

    #[test]
    fn parse_body_is_empty_when_no_prose() {
        let src = "---\nname: x\nmode: html\ndescription: d.\n---\n";
        let fm = SkillFrontMatter::parse(src).unwrap();
        assert_eq!(fm.body, "");
    }

    #[test]
    fn find_returns_skill_with_body() {
        let tmp = std::env::temp_dir().join("od-skill-body");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("html-page")).unwrap();
        fs::write(
            tmp.join("html-page").join("SKILL.md"),
            HTML_SKILL_WITH_BODY,
        )
        .unwrap();

        let skills = discover(&tmp, None);
        let s = find(&skills, "html-page").expect("html-page present");
        assert!(s.body().contains("Body line 1."));
        assert!(s.body().starts_with("# html-page"));

        fs::remove_dir_all(&tmp).ok();
    }
}
