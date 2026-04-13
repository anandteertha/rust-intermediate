use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use crate::metrics::Metrics;

mod metrics;
fn main() {
    let safe_metrics = Arc::new(Mutex::new(Metrics::new(0, 0)));

    let arc_metrics_a = Arc::clone(&safe_metrics);
    let worker_thread_a = spawn(move || {
        let m = arc_metrics_a.lock();
        match m {
            Ok(mut metric) => {
                metric.add_request();
                metric.add_error();
                metric.add_error();
                metric.add_request();
                metric.add_request();
            }
            Err(error) => {
                println!("Got an error while trying to lock metrics: {:?}", error);
            }
        }
    });
    let arc_metrics_b = Arc::clone(&safe_metrics);
    let worker_thread_b = spawn(move || {
        let m = arc_metrics_b.lock();
        match m {
            Ok(mut metric) => {
                metric.add_request();
                metric.add_request();
                metric.add_request();
            }
            Err(error) => {
                println!("Got an error while trying to lock metrics: {:?}", error);
            }
        }
    });
    let arc_metrics_c = Arc::clone(&safe_metrics);
    let worker_thread_c = spawn(move || {
        let m = arc_metrics_c.lock();
        match m {
            Ok(mut metric) => {
                metric.add_error();
                metric.add_request();
                metric.add_request();
            }
            Err(error) => {
                println!("Got an error while trying to lock metrics: {:?}", error);
            }
        }
    });

    worker_thread_a.join().unwrap();
    worker_thread_b.join().unwrap();
    worker_thread_c.join().unwrap();

    let final_metrics = safe_metrics.lock().unwrap();
    final_metrics.print_summary();
}
