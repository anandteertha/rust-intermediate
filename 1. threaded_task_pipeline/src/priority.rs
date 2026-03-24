#[derive(Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
    Unknown,
}

impl Priority {
    pub fn get_priority(text: &str) -> Priority {
        match text {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Unknown,
        }
    }
}
