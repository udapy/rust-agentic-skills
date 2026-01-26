use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub rpi_phase: String,
    pub trigger: Vec<String>,
    pub capabilities: Vec<String>,
    #[serde(skip)]
    pub path: PathBuf, // Absolute path to the skill directory
}

impl Skill {
    pub fn tool_name(&self) -> String {
        self.name.to_lowercase().replace(" ", "_")
    }

    pub fn tool_description(&self) -> String {
        format!(
            "{} [Trigger: {:?}] [Phase: {}]",
            self.description, self.trigger, self.rpi_phase
        )
    }
}

pub fn load_skills(project_root: &Path) -> Result<Vec<Skill>> {
    let skills_dir = project_root.join("skills");
    let mut skills = Vec::new();

    if !skills_dir.exists() {
        return Ok(skills);
    }

    for entry in fs::read_dir(skills_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let skill_md_path = path.join("SKILL.md");
            if skill_md_path.exists() {
                let content = fs::read_to_string(&skill_md_path)?;
                // Parse frontmatter (YAML between first two ---)
                if let Some(frontmatter) = extract_frontmatter(&content) {
                    let mut skill: Skill = serde_yaml::from_str(frontmatter)
                        .with_context(|| format!("Failed to parse SKILL.md for {:?}", path))?;
                    skill.path = path.clone();
                    skills.push(skill);
                }
            }
        }
    }

    Ok(skills)
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            return Some(parts[1]);
        }
    }
    None
}
