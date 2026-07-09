//! 内置技能模型：`SKILL.md` front matter 与正文。技能是文件系统目录，不是硬编码逻辑。
//!
//! Spec: docs/specs/built-in-skills.md

use crate::artifact::ArtifactKind;
use crate::error::{OdError, Result};
use crate::paths::ensure_within;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// `SKILL.md` front matter（M1 只要求 name / mode / description，额外字段忽略）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontMatter {
    pub name: String,
    /// `html` | `docs` | `slides`。
    pub mode: String,
    pub description: String,
    pub template: Option<String>,
    pub visual_brief: Option<String>,
    /// front matter 结束后的正文，供 `odl skill show` 输出给 agent 阅读。
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

    /// 解析 `SKILL.md` 顶部 YAML front matter。M1 实现。
    pub fn parse(source: &str) -> Result<Self> {
        let mut lines = source.lines();
        if lines.next() != Some("---") {
            return Err(invalid("missing opening front matter delimiter"));
        }

        let mut name = None;
        let mut mode = None;
        let mut description = None;
        let mut template = None;
        let mut visual_brief = None;
        let mut closed = false;

        let mut body_lines = Vec::new();
        for line in lines.by_ref() {
            if line.trim() == "---" {
                closed = true;
                break;
            }

            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            let value = unquote(value.trim());
            match key.trim() {
                "name" => name = Some(value.to_string()),
                "mode" => mode = Some(value.to_string()),
                "description" => description = Some(value.to_string()),
                "template" => template = Some(value.to_string()),
                "visualBrief" | "visual_brief" => visual_brief = Some(value.to_string()),
                _ => {}
            }
        }

        if !closed {
            return Err(invalid("missing closing front matter delimiter"));
        }

        body_lines.extend(lines);

        Ok(Self {
            name: required(name, "name")?,
            mode: required(mode, "mode")?,
            description: required(description, "description")?,
            template,
            visual_brief,
            body: body_lines.join("\n").trim().to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub front: SkillFrontMatter,
    pub root: PathBuf,
}

impl Skill {
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let root = fs::canonicalize(dir)?;
        let source = fs::read_to_string(root.join("SKILL.md"))?;
        let front = SkillFrontMatter::parse(&source)?;
        Ok(Self { front, root })
    }

    pub fn template_path(&self) -> Option<PathBuf> {
        let template = self.front.template.as_deref()?;
        let rel = Path::new(template);
        if !is_safe_relative_path(rel) || ensure_within(&self.root, rel).is_err() {
            return None;
        }
        Some(self.root.join(rel))
    }

    pub fn kind(&self) -> Option<ArtifactKind> {
        self.front.kind()
    }

    pub fn name(&self) -> &str {
        &self.front.name
    }

    pub fn body(&self) -> &str {
        &self.front.body
    }
}

/// 内置 dir + 可选 workspace dir，后者同名覆盖前者。
pub fn discover(builtin: &Path, workspace: Option<&Path>) -> Vec<Skill> {
    let mut skills = BTreeMap::new();
    collect_skills(builtin, &mut skills);
    if let Some(workspace) = workspace {
        collect_skills(workspace, &mut skills);
    }
    skills.into_values().collect()
}

pub fn for_kind(skills: &[Skill], kind: ArtifactKind) -> Option<&Skill> {
    skills.iter().find(|skill| skill.kind() == Some(kind))
}

pub fn find<'a>(skills: &'a [Skill], name: &str) -> Option<&'a Skill> {
    skills.iter().find(|skill| skill.name() == name)
}

fn collect_skills(root: &Path, skills: &mut BTreeMap<String, Skill>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if let Ok(skill) = Skill::from_dir(&path) {
            skills.insert(skill.front.name.clone(), skill);
        }
    }
}

fn required(value: Option<String>, field: &str) -> Result<String> {
    match value.filter(|value| !value.is_empty()) {
        Some(value) => Ok(value),
        None => Err(invalid(format!("missing field: {field}"))),
    }
}

fn invalid(reason: impl Into<String>) -> OdError {
    OdError::SkillFrontMatterInvalid(reason.into())
}

fn unquote(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(value)
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parses_front_matter_fields() {
        let source = "---\nname: html-page\nmode: html\ndescription: Create HTML.\ntemplate: \"templates/basic.html\"\nvisualBrief: studio\nignored: ok\n---\nBody";
        let front = SkillFrontMatter::parse(source).unwrap();

        assert_eq!(front.name, "html-page");
        assert_eq!(front.kind(), Some(ArtifactKind::Html));
        assert_eq!(front.template.as_deref(), Some("templates/basic.html"));
        assert_eq!(front.visual_brief.as_deref(), Some("studio"));
        assert_eq!(front.body, "Body");
    }

    #[test]
    fn body_is_empty_when_front_matter_has_no_prose() {
        let front = SkillFrontMatter::parse(
            "---\nname: html-page\nmode: html\ndescription: Create HTML.\n---\n",
        )
        .unwrap();

        assert_eq!(front.body, "");
    }

    #[test]
    fn rejects_missing_required_field() {
        let err = SkillFrontMatter::parse("---\nname: html-page\ndescription: Create HTML.\n---\n")
            .unwrap_err();

        assert_eq!(err.code(), "skill_front_matter_invalid");
        assert!(err.to_string().contains("missing field: mode"));
    }

    #[test]
    fn built_in_skill_front_matter_parses() {
        let html =
            SkillFrontMatter::parse(include_str!("../../../skills/html-page/SKILL.md")).unwrap();
        let docs =
            SkillFrontMatter::parse(include_str!("../../../skills/docs-polish/SKILL.md")).unwrap();
        let slides =
            SkillFrontMatter::parse(include_str!("../../../skills/slides-html/SKILL.md")).unwrap();

        assert_eq!(html.kind(), Some(ArtifactKind::Html));
        assert_eq!(docs.kind(), Some(ArtifactKind::Markdown));
        assert_eq!(slides.kind(), Some(ArtifactKind::Slides));
    }

    #[test]
    fn workspace_skill_overrides_builtin() {
        let temp = temp_root("skill-override");
        let builtin = temp.join("builtin");
        let workspace = temp.join("workspace");
        write_skill(&builtin, "html-page", "html", "Builtin");
        write_skill(&workspace, "html-page", "html", "Workspace");
        write_skill(&builtin, "slides-html", "slides", "Slides");

        let skills = discover(&builtin, Some(&workspace));

        assert_eq!(skills.len(), 2);
        assert_eq!(
            find(&skills, "html-page").unwrap().front.description,
            "Workspace"
        );
        assert_eq!(
            for_kind(&skills, ArtifactKind::Slides).unwrap().name(),
            "slides-html"
        );

        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn unsafe_template_path_returns_none() {
        let temp = temp_root("skill-template");
        write_skill_with_template(&temp, "html-page", "html", "../x.html");
        let skill = Skill::from_dir(&temp.join("html-page")).unwrap();

        assert_eq!(skill.template_path(), None);

        let _ = fs::remove_dir_all(temp);
    }

    fn write_skill(root: &Path, name: &str, mode: &str, description: &str) {
        write_skill_with_template(root, name, mode, "templates/basic.html");
        let path = root.join(name).join("SKILL.md");
        let source = fs::read_to_string(&path)
            .unwrap()
            .replace("Description", description);
        fs::write(path, source).unwrap();
    }

    fn write_skill_with_template(root: &Path, name: &str, mode: &str, template: &str) {
        let dir = root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!(
                "---\nname: {name}\nmode: {mode}\ndescription: Description\ntemplate: {template}\n---\n"
            ),
        )
        .unwrap();
    }

    fn temp_root(prefix: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        std::env::temp_dir().join(format!("od-core-{prefix}-{millis}"))
    }
}
