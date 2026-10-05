pub struct Job {
    producer_id: usize,
    job_id: usize,
}

impl Job {
    pub fn new(producer_id: usize, job_id: usize) -> Self {
        Self {
            producer_id,
            job_id,
        }
    }

    pub fn get_producer_id(&self) -> usize {
        self.producer_id
    }

    pub fn get_job_id(&self) -> usize {
        self.job_id
    }
}
