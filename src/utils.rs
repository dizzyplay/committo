/// Parse commit message response into candidates
/// If response contains multiple lines, split them into separate candidates
/// Otherwise, return the single response as one candidate
pub fn parse_commit_message_candidates(response: &str, expected_count: u32) -> Vec<String> {
    let trimmed = response.trim();
    
    // If only one candidate expected, return as is
    if expected_count == 1 {
        return vec![trimmed.to_string()];
    }
    
    // Split by lines and filter out empty lines
    let lines: Vec<String> = trimmed
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Remove common numbering patterns like "1. ", "- ", etc.
            let line = line.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ')' || c == '-' || c == '*' || c == ' ');
            line.trim().to_string()
        })
        .filter(|line| !line.is_empty())
        .collect();
    
    // If we got multiple lines, return them as candidates
    if lines.len() > 1 {
        lines
    } else {
        // If we only got one line but expected multiple, return it as the only candidate
        vec![trimmed.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_candidate() {
        let response = "fix: resolve authentication issue";
        let candidates = parse_commit_message_candidates(response, 1);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
    }

    #[test]
    fn test_multiple_candidates_with_numbers() {
        let response = "1. fix: resolve authentication issue\n2. feat: add login validation\n3. refactor: improve auth flow";
        let candidates = parse_commit_message_candidates(response, 3);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
        assert_eq!(candidates[1], "feat: add login validation");
        assert_eq!(candidates[2], "refactor: improve auth flow");
    }

    #[test]
    fn test_multiple_candidates_with_dashes() {
        let response = "- fix: resolve authentication issue\n- feat: add login validation\n- refactor: improve auth flow";
        let candidates = parse_commit_message_candidates(response, 3);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
        assert_eq!(candidates[1], "feat: add login validation");
        assert_eq!(candidates[2], "refactor: improve auth flow");
    }

    #[test]
    fn test_multiple_candidates_plain_lines() {
        let response = "fix: resolve authentication issue\nfeat: add login validation\nrefactor: improve auth flow";
        let candidates = parse_commit_message_candidates(response, 3);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
        assert_eq!(candidates[1], "feat: add login validation");
        assert_eq!(candidates[2], "refactor: improve auth flow");
    }

    #[test]
    fn test_single_line_but_multiple_expected() {
        let response = "fix: resolve authentication issue";
        let candidates = parse_commit_message_candidates(response, 3);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
    }

    #[test]
    fn test_empty_lines_filtered() {
        let response = "1. fix: resolve authentication issue\n\n2. feat: add login validation\n\n\n3. refactor: improve auth flow\n";
        let candidates = parse_commit_message_candidates(response, 3);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], "fix: resolve authentication issue");
        assert_eq!(candidates[1], "feat: add login validation");
        assert_eq!(candidates[2], "refactor: improve auth flow");
    }
}