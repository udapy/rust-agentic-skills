use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HEADER: &str = "# The Guild Roster (Auto-Generated)";

    let template = fs::read_to_string("scripts/AGENTS_TEMPLATE.md")?;
    
    // We'll simplisticly find all SKILL.md and parse their frontmatter.
    // For a real script we might use serde_yaml, but here we do string scanning to avoid dependencies/compilation weight for this simple task
    
    let mut rust_table = String::from("| Agent | Description | Trigger |\n| :--- | :--- | :--- |\n");
    let mut general_table = String::from("| Agent | Description | Trigger |\n| :--- | :--- | :--- |\n");

    let skills_dir = Path::new("skills");
    if skills_dir.exists() {
        for entry in fs::read_dir(skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let skill_file = path.join("SKILL.md");
                if skill_file.exists() {
                    let content = fs::read_to_string(&skill_file)?;
                    let (name, desc, trigger) = parse_frontmatter(&content);
                    
                    let row = format!("| **{}** | {} | {} |\n", name, desc, trigger);
                    
                    if path.to_string_lossy().contains("general-") {
                        general_table.push_str(&row);
                    } else {
                        rust_table.push_str(&row);
                    }
                }
            }
        }
    }

    let output = template
        .replace("{{RUST_SKILLS_TABLE}}", &rust_table)
        .replace("{{GENERAL_SKILLS_TABLE}}", &general_table);

    fs::write("AGENTS.md", output)?;
    println!("Successfully generated AGENTS.md");
    Ok(())
}

fn parse_frontmatter(content: &str) -> (String, String, String) {
    let mut name = String::new();
    let mut desc = String::new();
    let mut trigger = String::new();
    
    let lines: Vec<&str> = content.lines().collect();
    let mut in_yaml = false;
    let mut in_trigger = false;

    for line in lines {
        if line.trim() == "---" {
            if in_yaml { break; } // End of frontmatter
            in_yaml = true;
            continue;
        }
        if !in_yaml { continue; }

        if line.starts_with("name:") {
            name = line.replace("name:", "").trim().to_string();
        } else if line.starts_with("description:") {
            desc = line.replace("description:", "").trim().to_string();
        } else if line.starts_with("trigger:") {
            in_trigger = true;
        } else if in_trigger {
            if line.trim().starts_with("-") {
               if !trigger.is_empty() { trigger.push_str(", "); }
               let t_val = line.trim().trim_start_matches("-").trim().trim_matches('"');
               trigger.push_str(t_val);
            } else if line.trim().starts_with("capabilities:") || line.trim().starts_with("rpi_phase:") {
                in_trigger = false;
            }
        }
    }
    
    (name, desc, trigger)
}
