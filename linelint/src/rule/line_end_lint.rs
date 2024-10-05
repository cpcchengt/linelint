use crate::config::Config;
use crate::issue::Issue;
use crate::rule::{LintRule, LINE_END_RULE_NAME};

pub struct LineEndLint {}

impl LintRule for LineEndLint {
    fn name(&self) -> &'static str {
        LINE_END_RULE_NAME
    }

    fn check(&self, config: &Config, filename: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        if !content.ends_with(config.line_ending.get_ending(content)) {
            issues.push(Issue::new(
                LINE_END_RULE_NAME,
                filename,
                "File does not end with the expected line ending",
                content.lines().count(),
            ));
        }
        issues
    }

    fn format(&self, config: &Config, content: &str) -> String {
        let mut formatted_content = content.to_string();
        let line_ending = config.line_ending.get_ending(content);
        if !formatted_content.ends_with(line_ending) {
            formatted_content.push_str(line_ending);
        }
        formatted_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::line::LineEnding;

    #[test]
    fn test_line_end_lint_check_with_proper_ending() {
        let config = Config::new(LineEnding::Unix);
        let lint = LineEndLint {};
        let content = "This is a test file.\n";
        let filename = "test_file.rs";

        let issues = lint.check(&config, filename, content);
        assert!(
            issues.is_empty(),
            "There should be no issues when the file ends with the correct line ending"
        );
    }

    #[test]
    fn test_line_end_lint_check_with_incorrect_ending() {
        let config = Config::new(LineEnding::Unix);
        let lint = LineEndLint {};
        let content = "This is a test file.";
        let filename = "test_file.rs";

        let issues = lint.check(&config, filename, content);
        assert_eq!(
            issues.len(),
            1,
            "There should be one issue for missing line ending"
        );
        assert_eq!(
            issues[0].description,
            "File does not end with the expected line ending"
        );
    }

    #[test]
    fn test_line_end_lint_format_adds_missing_ending() {
        let config = Config::new(LineEnding::Unix);
        let lint = LineEndLint {};
        let content = "This is a test file.";

        let formatted = lint.format(&config, content);
        assert_eq!(
            formatted, "This is a test file.\n",
            "The formatted content should end with the correct line ending"
        );
    }

    #[test]
    fn test_line_end_lint_format_does_nothing_if_correct_ending() {
        let config = Config::new(LineEnding::Unix);
        let lint = LineEndLint {};
        let content = "This is a test file.\n";

        let formatted = lint.format(&config, content);
        assert_eq!(
            formatted, content,
            "The content should not be changed if the correct line ending is present"
        );
    }
}