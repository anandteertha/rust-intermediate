pub struct Document {
    title: String,
    body: String,
    version: usize,
}

impl Document {
    pub fn new(title: String, body: String, version: usize) -> Self {
        Self {
            title,
            body,
            version,
        }
    }

    pub fn update_title(&mut self, title: &str) {
        self.title = title.to_owned();
        self.version += 1;
    }

    pub fn append_body(&mut self, new_paragraph: &str) {
        self.body.push_str(new_paragraph);
        self.version += 1;
    }

    fn word_count(&self) -> usize {
        self.body.split_whitespace().count()
    }

    fn char_count(&self) -> usize {
        self.body.chars().count()
    }

    pub fn display(&self) {
        println!(
            "**********************\ndocument title: {}\n**********************",
            self.title
        );
        println!("{}\nversion:{}", self.body, self.version);
        println!(
            "summary:\n# of words: {}\n# of chars: {}\n\n",
            self.word_count(),
            self.char_count()
        );
    }
}
