use std::collections::HashMap;

/// D0 Core standard library: Fundamental types, strings, collections, and error handling.
pub struct D0Core;

impl D0Core {
    /// Format text with arguments (like D0 `print` / `format`)
    pub fn format_text(template: &str, vars: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (k, v) in vars {
            let pattern = format!("{{{}}}", k);
            result = result.replace(&pattern, v);
        }
        result
    }

    /// Slice a string safely by character indices
    pub fn string_substring(s: &str, start: usize, len: usize) -> String {
        s.chars().skip(start).take(len).collect()
    }

    /// Trim whitespace
    pub fn string_trim(s: &str) -> String {
        s.trim().to_string()
    }

    /// Split string by delimiter
    pub fn string_split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|item| item.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_string_operations() {
        let s = "  Hello, D0!  ";
        assert_eq!(D0Core::string_trim(s), "Hello, D0!");

        let parts = D0Core::string_split("apple,banana,cherry", ",");
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "apple");

        let sub = D0Core::string_substring("Programming", 0, 7);
        assert_eq!(sub, "Program");
    }

    #[test]
    fn test_core_format_text() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Dinesh".to_string());
        vars.insert("lang".to_string(), "D0".to_string());

        let res = D0Core::format_text("Hello {name}, welcome to {lang}!", &vars);
        assert_eq!(res, "Hello Dinesh, welcome to D0!");
    }
}
