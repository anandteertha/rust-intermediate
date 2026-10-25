use std::{
    println,
    sync::{
        Arc, Mutex,
        mpsc::{self, sync_channel},
    },
    thread::spawn,
    time::Instant,
    vec,
};

use crate::{job::Job, job_result::JobResult};

mod job;
mod job_result;

fn main() {
    let (tx, rx) = sync_channel::<Job>(2);
    let offset: usize = 5;
    let mut producer_handlers = vec![];
    for producer_id in 1..=3 {
        let producer = tx.clone();
        let handler = spawn(move || {
            for job_id in 1..=5 {
                println!(
                    "Attempting for producer_id: {} and job_id: {}",
                    producer_id, job_id
                );
                let start = Instant::now();
                producer
                    .send(Job::new(
                        job_id + (producer_id - 1) * offset,
                        producer_id,
                        (job_id * producer_id).try_into().unwrap(),
                    ))
                    .unwrap();
                println!(
                    "submitted for producer_id: {}, job id: {} and {:?}",
                    producer_id,
                    job_id,
                    start.elapsed()
                );
            }
        });
        producer_handlers.push(handler);
    }

    drop(tx);

    let safe_rx = Arc::new(Mutex::new(rx));
    let mut worker_handlers = vec![];
    let (job_result_tx, job_result_rx) = mpsc::channel::<JobResult>();
    for worker_id in 1..=3 {
        let worker = safe_rx.clone();
        let result_sender_clone = job_result_tx.clone();
        let handler = spawn(move || {
            loop {
                let job;
                let m = worker.lock();
                match m {
                    Ok(receiver_guard) => {
                        job = receiver_guard.recv();
                    }
                    Err(err) => {
                        println!("not able to handle the receive guard: {:?}", err);
                        return;
                    }
                }
                match job {
                    Ok(mut job) => {
                        let outcome;
                        if job.get_job_id() % 5 == 0 {
                            outcome = Err("A failure occurred".to_string());
                        } else {
                            outcome = Ok(job.square_value());
                        }
                        let job_result = JobResult::new(job.get_job_id(), worker_id, outcome);
                        match result_sender_clone.send(job_result) {
                            Ok(_) => println!("successfully sent the job result to it's channel"),
                            Err(err) => {
                                println!(
                                    "an error occurred when sending the job to its channel: {:?}",
                                    err
                                );
                                return;
                            }
                        }
                    }
                    Err(_) => {
                        println!("worker finished: no more jobs.");
                        break;
                    }
                }
            }
        });
        worker_handlers.push(handler);
    }

    drop(job_result_tx);
    let mut success: usize = 0;
    let mut failure: usize = 0;
    while let Ok(job_result) = job_result_rx.recv() {
        if job_result.is_outcome_success() {
            success += 1;
        } else {
            failure += 1;
        }
        println!(
            "total: {}, success: {}, failure: {}",
            success + failure,
            success,
            failure
        );
        job_result.print_summary();
    }

    for producer_handler in producer_handlers {
        match producer_handler.join() {
            Ok(_) => println!("successfully joined producer"),
            Err(err) => println!("thread panicked or faced an error: {:?}", err),
        }
    }
    for worker_handler in worker_handlers {
        match worker_handler.join() {
            Ok(_) => println!("successfully joined worker"),
            Err(err) => println!("thread panicked or faced an error: {:?}", err),
        }
    }
}
