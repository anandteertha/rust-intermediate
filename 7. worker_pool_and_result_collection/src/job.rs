pub struct Job {
    job_id: usize,
    producer_id: usize,
    value: u32,
}

impl Job {
    pub fn new(job_id: usize, producer_id: usize, value: u32) -> Self {
        Self {
            job_id,
            producer_id,
            value,
        }
    }

    pub fn get_job_id(&self) -> usize {
        self.job_id
    }

    pub fn square_value(&mut self) -> u32 {
        self.value *= self.value;
        self.value
    }
}
