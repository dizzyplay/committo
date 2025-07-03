use std::env;
use std::fs;
use std::io;

/// Find and build hierarchical prompt from .committoconvention files
pub fn find_and_build_prompt() -> io::Result<String> {
    let mut prompt_parts = Vec::new();
    let current_dir = env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        let convention_path = ancestor.join(".committoconvention");
        if convention_path.exists() {
            let content = fs::read_to_string(convention_path)?;
            prompt_parts.push(content);
        }
    }

    // The prompts are found from child to parent, so we reverse to get parent-to-child order.
    prompt_parts.reverse();
    
    // Add priority numbers (1 = highest, 2, 3, 4... = lower priority)
    let numbered_prompts: Vec<String> = prompt_parts
        .into_iter()
        .enumerate()
        .map(|(i, content)| format!("{}. {}", i + 1, content.trim()))
        .collect();
    
    Ok(numbered_prompts.join("\n\n"))
}