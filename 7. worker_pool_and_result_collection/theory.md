# Project 007 — Theory: Worker Pools and Result Collection

## 1. Begin with the gap left by a bounded queue

Project 006 had several producers and one consumer. A capacity-two channel limited buffered jobs and made producers wait when that buffer was full. However, one consumer still processed jobs one at a time.

Project 007 keeps that bounded admission path and introduces three workers. A worker is a long-lived thread that repeatedly receives and handles jobs. Creating three workers once lets the program reuse those threads for fifteen jobs instead of creating a thread for every submission.

The two controls have different purposes. Queue capacity determines how many jobs can wait inside the channel. Worker count determines how many jobs can be handled concurrently. Increasing one does not automatically improve the other.

The running example is fifteen integer-squaring jobs. IDs 5, 10, and 15 deliberately fail. Every job still needs an identified outcome so main can distinguish successful work, expected failures, and absent results.

## 2. Follow one job through the whole system

Producer 2 creates its second job:

```text
global ID = 2 + (2 - 1) * 5 = 7
input value = 2 * 2 = 4
```

It sends that `Job` by value through the bounded channel. Once the send succeeds, the producer cannot use that moved value. A worker receives ownership of it, so the worker can mutate it without sharing that job with the other workers.

Suppose worker 3 receives job 7. It releases the receiver guard, checks the global ID, squares the input to 16, and creates a `JobResult` containing ID 7, worker ID 3, and `Ok(16)`. Sending the record transfers it toward the collector. Main receives it, increments the success counter, and prints its summary.

For job 10, the receive succeeds in exactly the same way. The worker constructs an outcome such as `Err("A failure occurred".to_string())` and sends it. A failed calculation is still a completed attempt with a reportable outcome. It does not mean that receiving failed or that the worker should stop.

## 3. Why there are two channels

| Channel | Senders | Receiving side | Meaning of a message |
| --- | --- | --- | --- |
| Bounded job channel | Producers | Workers coordinate access to one receiver | Work to attempt |
| Unbounded result channel | Workers | Main alone | Outcome of one attempted job |

A successful buffered job send confirms acceptance by the channel. It does not confirm execution. A result is the application's later report about execution.

The result record contains `Result<u32, String>` because both success and failure are ordinary data. Its outer identity fields remain useful in either case:

```rust
pub struct JobResult {
    job_id: usize,
    worker_id: usize,
    outcome: Result<u32, String>,
}
```

The producer ID is not required in this result because the global job ID already identifies the submission. A richer application might retain more metadata, but this exercise does not need it for counting outcomes.

## 4. Receiver, mutex, guard, and job are different values

These names describe different responsibilities:

| Value | Responsibility |
| --- | --- |
| `Receiver<Job>` | Receive jobs from one channel |
| `Mutex<Receiver<Job>>` | Allow one thread at a time to access that receiver through a guard |
| `Arc<Mutex<Receiver<Job>>>` | Give several threads shared ownership of the same mutex and receiver |
| Mutex guard | Keep the mutex locked during an access scope |
| `Job` | One owned message removed from the channel |

The standard `mpsc` receiver cannot simply be cloned for multiple workers, and it is not `Sync`: references to it cannot be shared directly across threads in this way. The mutex coordinates access. `Arc` supplies shared ownership across thread lifetimes; cloning an `Arc` points at the same allocation rather than copying the receiver or its jobs.

`Arc` alone does not make arbitrary inner values safe to access concurrently. Here the mutex provides the coordinated access required by the receiver. The `Job` fields are integers, so the job can move across threads. These pieces satisfy distinct ownership and access requirements.

When a worker calls `lock()`, the successful result contains a guard. While that guard exists, another worker attempting to lock must wait. Dropping the guard releases the lock automatically. `recv()` removes a message and returns an owned job; that job does not borrow the guard and can outlive it.

## 5. Receiving is serialized; processing can overlap

Imagine jobs A, B, and C are available:

| Step | Worker 1 | Worker 2 | Worker 3 |
| --- | --- | --- | --- |
| 1 | Lock and receive A | Wait for lock | Wait for lock |
| 2 | Release lock; process A | Lock and receive B | Wait for lock |
| 3 | Process A | Release lock; process B | Lock and receive C |
| 4 | Process A | Process B | Release lock; process C |

Only one worker accesses the receiver at a time, but several workers can own different jobs and process them concurrently. Actual simultaneous execution depends on available hardware and scheduling.

If a worker kept its guard while processing A, workers 2 and 3 could not receive B and C during that work. Three threads would exist, but the expensive part would effectively remain serialized behind the receiver lock.

When the queue is empty, the worker inside `recv()` waits while still holding the receiver guard. Other workers may wait for the mutex. This design does not provide three independent blocking receivers, but it still permits overlapping processing after successful receives.

## 6. Make guard lifetime visible

The exercise used this pattern inside a spawned worker:

```rust
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

// The successful match arm ended, so receiver_guard has dropped.
match job {
    Ok(mut job) => {
        // Process the owned job here, outside the receiver lock.
    }
    Err(_) => break,
}
```

This is a fragment of the worker loop, not a standalone program. The important boundary is the end of the arm containing `receiver_guard`. Moving the guard out of `m` into that arm means the guard is dropped when the arm ends; keeping the binding `m` in the surrounding scope does not keep that moved guard alive.

A separate block can also make the boundary explicit:

```rust
let received = {
    let guard = match worker.lock() {
        Ok(guard) => guard,
        Err(err) => {
            println!("receiver lock failed: {:?}", err);
            return;
        }
    };
    guard.recv()
};
// Process received after the guard has dropped.
```

The block's final expression is the receive result. The guard stays inside the block. This is an alternative expression of the same principle, not a required rewrite.

Avoid assuming that a guard inside a compact `while let` condition is released before the body. Temporary lifetimes can retain it through the loop body. An explicit scope is easier to inspect when the purpose is to process outside the lock.

## 7. What receiving returns

`recv()` produces `Result<Job, RecvError>`:

| Channel state | Behavior |
| --- | --- |
| A buffered message exists | Return `Ok(job)` |
| Empty, with at least one sender alive | Wait |
| Empty, with no senders alive | Return `Err(RecvError)` |

Dropping senders does not discard already buffered messages. Workers drain those messages before receiving reports disconnection.

The inner worker loop therefore repeats until `recv()` reports that no more messages can arrive. A fixed loop of three receives per worker would cap total attempts at nine. Even five receives per worker assumes a distribution the scheduler does not promise. The stopping rule belongs to channel state, not a per-worker quota.

`break` exits the worker's receive loop. `return` exits the entire worker closure. Neither exits main, because it is executing on another thread. In this implementation, the lock-error branch returns; the disconnected-receive branch breaks.

## 8. Preserve a Result instead of unwrapping it too early

Consider `Ok(receiver_guard.recv().unwrap())`.

On success, `recv()` returns `Ok(job)`, `unwrap()` extracts the job, and the outer `Ok` wraps it again. On disconnection, `unwrap()` panics before that outer wrapper exists. A later `match` cannot handle an error value that was already converted into a panic.

Keeping `receiver_guard.recv()` intact preserves both possibilities for ordinary control flow. This is useful because disconnection is the expected ending of the worker's input stream.

A panic while holding a mutex guard can also poison that mutex under the usual unwinding behavior. Subsequent locking then reports poisoning. Poisoning is advisory evidence that a panic occurred during protected access; recovery is possible, but requires an explicit policy. This project logs the lock error and exits the worker rather than attempting recovery. With panic-abort behavior, the process can terminate instead of unwinding to a join error.

`unwrap()` expresses an assumption that an error should not occur on this path. It is not universally wrong, but it is a poor fit for an expected shutdown signal.

## 9. Why a string cannot replace a receive error

Once `job` receives the value returned by `recv()`, its type is `Result<Job, RecvError>`. Its error slot must contain a `RecvError`. `Err("job error")` contains a string slice and has a different type.

More fundamentally, locking and receiving represent different stages. A lock error is about access to protected state; a receive error is about the input stream ending. Assigning a fabricated receive error to represent a lock failure would hide that distinction.

Instead, the lock-error branch logs and returns. The successful branch initializes `job`. Every path that reaches the later `match job` has therefore assigned it. If the error branch only printed and continued, Rust would correctly reject the potentially uninitialized use.

Definite initialization is a control-flow rule: a variable must be initialized on every path that reaches its use. A path that exits beforehand does not need to initialize it.

## 10. Four failure layers and a normal ending

| Event | Meaning | Current response |
| --- | --- | --- |
| Job outcome is `Err(String)` | This job deliberately failed | Send the failure record; continue |
| Receiving returns `Err(RecvError)` | Input is disconnected and drained | End the worker normally |
| Lock returns a poison error | Protected access was affected by a panic | Log and return |
| Result send fails | Main's result receiver is gone | Log and return |
| `join()` returns `Err` | The joined thread panicked while unwinding | Report the panic |

These outcomes must not be collapsed into one generic “error.” In particular, `join() == Ok(())` means a thread ended without a panic; it does not mean every job succeeded. A worker that deliberately reports failed jobs can join normally. A worker that logs an infrastructure error and returns can also join normally, potentially with missing results.

`unwrap_or` on a join returning `()` would substitute an empty fallback and could hide the panic. Matching lets the program report it. This example logs failures; it does not restart workers or guarantee progress after infrastructure failure.

## 11. Channel ownership is the shutdown protocol

Normal shutdown follows ownership in this order:

1. Main drops the original job sender after giving clones to producers.
2. Producers finish submitting, then their sender clones drop.
3. Workers receive remaining jobs, then observe the disconnected, empty input.
4. Worker closures finish, dropping their result-sender clones.
5. Main drains remaining results, then its receive loop ends.
6. Main joins every producer and worker to inspect thread completion.

Main must also drop the original result sender after spawning workers. Retaining it would keep the result channel connected after all workers finish; main could wait forever for a message no worker can send.

Dropping a sender does not wait for work or join any thread. Conversely, joining a thread does not close sender clones owned elsewhere.

The original `safe_rx` remains alive on main in the reviewed implementation. If all workers exit early while producers are blocked, that remaining receiver ownership can keep producers blocked instead of giving them a disconnected-channel error. Handling every infrastructure-failure path would require additional shutdown design. The normal observed run did not exercise that failure case.

## 12. Why main collects before joining

The result channel is unbounded, so workers can enqueue this small set of results even if main waits to receive until after joining them. That order works for the demonstrated finite workload, but buffers all results first.

Collecting while workers run gives main outcomes as they become available and drains the output queue sooner. It also avoids a specific deadlock if the output channel is later made bounded: main waits for a worker to join, while that worker waits for main to receive and free an output slot.

All producers and workers must be spawned before main begins this collection phase. Joining producers before workers have been started could similarly block on a full job queue with nobody available to drain it.

`spawn` starts a thread independently. `join` waits for termination; it does not start execution. Joining several already spawned threads in sequence does not make their work sequential. Dropping a join handle detaches its thread; it does not wait. Returning from main ends the program, so storing handles without joining them is insufficient to keep unfinished work alive.

## 13. Count outcomes without consuming the result

The implemented method is:

```rust
pub fn is_outcome_success(&self) -> bool {
    self.outcome.is_ok()
}
```

`&self` borrows the record. The method inspects its outcome without moving it or cloning the error string, so the collector can still call `print_summary()` afterward.

Only main updates the success and failure counters. Workers communicate through their channel instead of directly changing those counters, so no application mutex is needed around them. Total results can be computed as `success + failure` because each received record increments exactly one counter.

Printing inside the receive loop shows running totals. Printing after the loop shows the final summary.

For fifteen results, the expected split is twelve successes and three deliberate failures. However, a duplicate job 7 replacing missing job 8 could leave these counts unchanged. Optional stronger verification would track IDs in a `HashSet`, reject repeats, compare with 1–15, and check values and expected failure IDs. Such checks verify this execution; they do not create a durable exactly-once processing guarantee.

## 14. Read logs without inventing an execution order

“Attempting” is printed before sending; it does not reserve a queue position. “Submitted” is printed after sending. A worker can receive and report a job before the producer gets CPU time to print its post-send message.

Likewise, worker A may receive before worker B but finish later. Main sees result-channel arrival order, not sorted job-ID order or necessarily input dequeue order. Different workers may handle very different numbers of jobs. The implementation promises neither equal distribution nor round-robin fairness.

Global IDs help connect events despite this interleaving. The producer's current logs show local job IDs, while result logs show global IDs; readers must account for that difference when tracing output.

## 15. Capacity and memory still need separate reasoning

Capacity two means at most two jobs buffered in the input channel. Three workers may each retain an active job, and three producers may each retain a job in a blocked send. Under that specific one-job-at-a-time discipline, as many as eight input jobs can be retained across these locations: `2 + 3 + 3`.

That is not a whole-process memory bound. Thread stacks, synchronization metadata, retained inputs, processing workspace, and results consume memory too. The unbounded result channel can accumulate outputs if workers outpace main. A bounded input alone cannot bound every stage of a pipeline.

The current inputs are small integers. For large heap-backed payloads, count allocations and retained capacities, not just `size_of::<Job>()`. Project 006 discusses byte admission in more depth; implementing it is outside this chapter.

## 16. Concurrency does not establish a speedup

The project demonstrates overlapping worker lifetimes and correct result flow. It does not measure a performance improvement. Tiny integer squaring is likely dominated by scheduling, locking, channels, and logging.

For intuition only, fifteen independent equal-duration jobs taking time `s` need `15s` on one worker. With three continuously busy workers and negligible overhead, five rounds would take about `5s`. If `s` were 200 ms, that model suggests about three seconds versus one second. Those are analytical estimates, not measurements of this implementation. Deliberate failures that skip work would also change the model.

A useful optional experiment would compare one and three workers on the same work, preserve the same failure behavior, minimize logging, time the full workload consistently, repeat runs, and verify the outcomes. CPU work and sleeping delays test different constraints. More workers can eventually add contention or exceed available CPU resources.

Send timing currently starts before job construction and includes conversion, sending, any blocking, and scheduling delay. It measures submission duration, not end-to-end job latency or exact time spent waiting for buffer space. The time Cargo spends compiling is separate from workload runtime.

## 17. Check your understanding

Try explaining these without looking back. Worked answers are in [interview.md](interview.md).

1. Why can three workers process concurrently if only one can receive at a time?
2. What exact event releases the receiver mutex in the two-match pattern?
3. What happens when `recv()` sees an empty queue with one sender still alive?
4. Why does a failed job need a `JobResult` rather than a worker exit?
5. Why does the lock-error branch need `return` if it does not initialize `job`?
6. Which retained sender could make main wait forever after workers finish?
7. How could all threads join normally while some expected jobs remain unreported?
8. Why do 15 results and the correct success count not prove unique coverage?
9. Why is neither queue capacity nor thread count a whole-process memory limit?
10. What evidence would justify a speedup claim?

The core exercise is complete. Automatic verification and performance experiments extend the evidence; they are not hidden prerequisites for finishing this lesson.

## Official references

- [Receiver: recv and trait implementations](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
- [Mutex: guards and poisoning](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [MutexGuard](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html)
- [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html)
- [channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)
- [Result](https://doc.rust-lang.org/std/result/enum.Result.html)
- [thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [JoinHandle::join](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
- [Rust Reference: destructors and temporary scopes](https://doc.rust-lang.org/reference/destructors.html)

API rules follow the official references. Workload traces and calculations are examples with their assumptions stated; observed behavior refers to the reviewed console run.
