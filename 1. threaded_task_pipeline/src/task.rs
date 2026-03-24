use crate::priority::Priority;

#[derive(Debug)]
pub struct Task {
    id: String,
    description: String,
    duration: usize,
    priority: Priority,
}

impl Task {
    fn get_id(words: &[&str]) -> String {
        words[0].to_owned()
    }
    fn get_description(words: &[&str]) -> String {
        words[1].to_owned()
    }
    fn get_duration(words: &[&str]) -> Result<usize, std::num::ParseIntError> {
        words[2].parse::<usize>()
    }
    fn get_priority(words: &[&str]) -> String {
        words[3].to_owned()
    }
    pub fn new(text: &str) -> Self {
        let words: Vec<&str> = text.split('|').collect();
        let id: String = Self::get_id(&words);
        let description: String = Self::get_description(&words);
        let duration: usize = Self::get_duration(&words).unwrap_or(0);
        let priority: String = Self::get_priority(&words);

        let priority = Priority::get_priority(&priority);
        Self {
            id,
            description,
            duration,
            priority,
        }
    }
    pub fn print_summary(&self) {
        println!(
            "{:?}: {}-{} is valid till {}",
            self.priority, self.id, self.description, self.duration
        )
    }
}
