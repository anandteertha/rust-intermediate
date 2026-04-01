pub enum Severity {
    High,
    Medium,
    Low,
    Unknown,
}

impl Severity {
    pub fn get_severity(severity_string: &str) -> Self {
        let severity_string = severity_string.to_lowercase();
        match severity_string.as_str() {
            "high" => Self::High,
            "low" => Self::Low,
            "medium" => Self::Medium,
            _ => Self::Unknown,
        }
    }
}
