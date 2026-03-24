use crate::{file_helper::load_text, task::Task};
mod file_helper;
mod priority;
mod task;

fn main() {
    // create a channel.
    let (tx, rx) = std::sync::mpsc::channel::<Task>();

    // create a worker thread.
    let handle = std::thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            task.print_summary();
        }
    });

    // create tasks.
    let tasks_result = load_text(
        "C:/Users/anand/Development/rust-intermediate/threaded_task_pipeline/src/sample_tasks.txt",
    );

    match tasks_result {
        Ok(tasks) => {
            for task_string in tasks.lines() {
                let task = Task::new(task_string);
                let task_result = tx.send(task);
                match task_result {
                    Ok(_) => {}
                    Err(error) => {
                        println!("error occurred: {}", error);
                    }
                }
            }
        }
        Err(error) => println!("{}", error),
    }
    drop(tx);
    let handle_result = handle.join();
    match handle_result {
        Ok(_) => {}
        Err(error) => {
            println!("error occurred: {:?}", error)
        }
    }
}
