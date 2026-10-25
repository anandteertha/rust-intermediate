use std::println;

pub struct JobResult {
    job_id: usize,
    worker_id: usize,
    outcome: Result<u32, String>,
}

impl JobResult {
    pub fn new(job_id: usize, worker_id: usize, outcome: Result<u32, String>) -> Self {
        Self {
            job_id,
            worker_id,
            outcome,
        }
    }

    pub fn print_summary(&self) {
        println!(
            "job id: {}, worker id: {} has outcome: {:?}",
            self.job_id, self.worker_id, self.outcome
        );
    }

    pub fn is_outcome_success(&self) -> bool {
        self.outcome.is_ok()
    }
}
