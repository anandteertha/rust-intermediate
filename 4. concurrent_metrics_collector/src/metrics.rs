pub struct Metrics {
    errors: usize,
    requests: usize,
}

impl Metrics {
    pub fn new(errors: usize, requests: usize) -> Self {
        Self { errors, requests }
    }

    pub fn add_error(&mut self) {
        self.errors += 1;
    }
    pub fn add_request(&mut self) {
        self.requests += 1;
    }
    pub fn print_summary(&self) {
        println!(
            "Total requests handled:{}\nTotal errors: {}",
            self.requests, self.errors
        );
    }
}
