pub struct D0Data;

impl D0Data {
    pub fn parse_csv_line(line: &str) -> Vec<String> {
        line.split(',').map(|s| s.trim().to_string()).collect()
    }

    pub fn format_csv_line(fields: &[&str]) -> String {
        fields.join(",")
    }
}
