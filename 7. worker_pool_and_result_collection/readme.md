# Project 007 — Worker Pool and Result Collection

A Rust concurrency exercise with three producers, a bounded job queue, three persistent workers, and one result collector on the main thread.

## What this project teaches

Project 006 controlled how much work could wait in a queue. Project 007 adds processing concurrency and a way to report each job's outcome back to main.

The central idea is to share access to the receiver briefly, then process each owned job outside the lock. Workers remain alive to receive more jobs, including after a deliberate job failure. Channel ownership determines when collection ends; thread joins confirm termination.

## Implemented behavior

| Setting | Value |
| --- | --- |
| Producers | 3 operating-system threads |
| Jobs per producer | 5 |
| Global job IDs | 1–15 |
| Buffered job capacity | 2 jobs, using `sync_channel` |
| Workers | 3 operating-system threads |
| Receiver coordination | `Arc<Mutex<Receiver<Job>>>` |
| Successful processing | Square the job's input value |
| Deliberate failures | Global IDs 5, 10, and 15 |
| Results | Unbounded `mpsc::channel<JobResult>` |
| Collector | Main thread; prints results and counts outcomes |
| Shutdown | Drop original senders, drain results, join producers and workers |

Each producer submits local IDs 1–5. A global ID is calculated as `local_id + (producer_id - 1) * 5`; the input value is `local_id * producer_id`. For example, global job 7 belongs to producer 2, has local ID 2 and input 4, and should return 16. Job 10 deliberately fails instead of being squared.

## Files

- `src/main.rs`: channels, producer and worker spawning, result collection, counters, and joins.
- `src/job.rs`: private job fields, constructor, accessors, and `square_value()`.
- `src/job_result.rs`: job ID, worker ID, and `Result<u32, String>`, with construction and inspection methods.
- [theory.md](theory.md): execution traces, ownership, lock scope, errors, shutdown, and performance limits.
- [interview.md](interview.md): concise explanations, debugging questions, and worked answers.

These documentation files belong beside `Cargo.toml` in the project directory.

## Run

From the repository root:

```sh
cd "7. worker_pool_and_result_collection"
cargo run
```

The implementation uses only the Rust standard library. This documentation describes the implementation and console output reviewed during the exercise; it does not introduce a replacement implementation.

## Execution and ownership

Main creates the bounded job channel and gives each producer a cloned sender. After spawning producers, it drops its original job sender. Each producer submits five owned jobs and drops its sender when its closure finishes.

Main puts the single job receiver inside a mutex shared through `Arc`. Each worker gets an `Arc` clone and a result-sender clone. Inside its loop, a worker locks the receiver, receives one job, and releases the guard before inspecting and processing that job. A job ID divisible by five produces an error outcome; any other ID produces the squared value. Both paths send one `JobResult` and continue.

After spawning workers, main drops its original result sender and collects results while workers run. Once all worker result senders are dropped and buffered results have been drained, collection ends. Main then joins every producer and every worker, reporting thread panics separately from job failures.

## Observed results

The reviewed console run contained:

| Check | Observed |
| --- | --- |
| Results | 15 |
| Successes | 12 |
| Deliberate failures | 3, belonging to IDs 5, 10, and 15 |
| IDs | Every ID from 1 through 15 appeared once, checked manually |
| Successful values | Matched the squared inputs in the displayed run |
| Joins | All three producers and all three workers joined normally |

Workers did not receive equal numbers of jobs. Result order also differed from numeric ID order. Scheduling can change both on every run.

Three final `RecvError` messages represented normal worker shutdown: the job channel was disconnected and empty. Wording such as “worker finished: no more jobs” would communicate that more clearly. The warning about the unread `producer_id` field concerns unused metadata, not a failed job.

## Scope and limitations

The core learning scope is complete: persistent workers, coordinated receiving, processing outside the lock, outcome collection, separate failure layers, and thread lifecycle management.

Automatic duplicate/missing/unexpected ID checks and value assertions remain optional verification work. Counts alone cannot prove that every intended job was handled exactly once. A timing comparison was not performed, so this project makes no measured speedup claim. Squaring a small integer is too little work to justify thread overhead by itself.

The queue limits buffered job count, not total process memory. The result channel has no configured capacity limit. There is no retry policy, persistence, cancellation, worker restart, or guaranteed fair job distribution. Producer sends still use `unwrap()`. Workers report result-send failures and exit. The receiver remains shared with main through its original `Arc`, so losing all workers does not necessarily disconnect blocked producers immediately; this is an educational happy-path design with explicit error reporting, not a fault-tolerant service.

`square_value()` mutates its `u32` input in place. The exercise's small values fit safely; arbitrary inputs would need an overflow policy.

## Reading

- [Theory](theory.md)
- [Interview preparation](interview.md)
- [Rust: Receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
- [Rust: Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Rust: Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Rust: channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)
- [Rust: JoinHandle](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
