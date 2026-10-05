mod job;
use crate::job::Job;
use std::{
    println,
    sync::mpsc::sync_channel,
    thread::{sleep, spawn},
    time::{Duration, Instant},
    vec,
};

fn main() {
    let (tx, rx) = sync_channel::<Job>(2);

    let mut producer_handlers = vec![];

    for producer_id in 1..=3 {
        let producer = tx.clone();
        let producer_handler = spawn(move || {
            for job_id in 1..=5 {
                println!(
                    "Attempting for producer_id: {} and job_id: {}",
                    producer_id, job_id
                );
                let start = Instant::now();
                producer.send(Job::new(producer_id, job_id)).unwrap();
                println!(
                    "submitted for producer_id: {}, job id: {} and {:?}",
                    producer_id,
                    job_id,
                    start.elapsed()
                );
            }
        });
        producer_handlers.push(producer_handler);
    }

    drop(tx);
    while let Ok(job) = rx.recv() {
        println!(
            "started: producer id: {} and job id: {}",
            job.get_producer_id(),
            job.get_job_id()
        );
        sleep(Duration::from_millis(200));
        println!(
            "completed: producer id: {} and job id: {}",
            job.get_producer_id(),
            job.get_job_id()
        );
    }

    while producer_handlers.len() > 0 {
        let handler = producer_handlers.pop().unwrap();
        handler.join().unwrap();
    }
}
