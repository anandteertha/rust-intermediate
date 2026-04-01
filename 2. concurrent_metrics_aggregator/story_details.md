# Rust Intermediate Project 002 - Rebuild

## Final project identity

### Folder name
002_concurrent_metrics_aggregator

### Project name
Concurrent Metrics Aggregator

### One-line repo subtitle
Learn shared-state concurrency in Rust by building a thread-safe service metrics aggregator.

---

## Why this should be Project 002

Project 001 already introduced one side of concurrency through a threaded pipeline and message flow.

Project 002 should deliberately teach the other major concurrency model:

- Project 001: ownership moving through a pipeline
- Project 002: multiple threads sharing and updating one common state safely

That makes 002 a natural and strong next step.

It also keeps the progression clean:

- first learn how threads communicate
- then learn how threads coordinate around shared state

---

## Why this should be a mini project and not a tiny concept demo

This project contains several important Rust ideas that deserve real design, not a toy one-counter example.

You are not just proving that a mutex exists.
You are building a small systems-flavored component that feels like something a backend or observability pipeline might actually need.

This project teaches all of the following together:

- thread spawning
- moving ownership into worker threads
- shared ownership with `Arc`
- synchronized mutation with `Mutex`
- lock scope and contention
- aggregation design
- deterministic final state despite nondeterministic execution order
- waiting for workers with `join`

That is enough substance for a mini project.

---

## Project story

Imagine a backend platform receiving runtime events from different services.

Examples:

- Auth login failed
- Payments timeout
- Database reconnection succeeded
- Cache miss
- API latency warning

Multiple worker threads process these events in parallel.
But instead of each worker printing isolated output, they all contribute to one shared metrics dashboard.

At the end, the program prints a final summary such as:

- total events processed
- total failures
- count by service
- count by severity
- critical event count

This makes the project feel like a tiny monitoring or observability subsystem.

---

## Core learning goal

Learn how Rust safely handles shared mutable state across threads.

The real question this project answers is:

How can many threads update one common data structure without creating race conditions?

---

## Main concepts this project should teach

### 1. Shared-state concurrency
Workers do not just send results away. They all touch the same shared metrics object.

### 2. `Arc`
Many threads need shared ownership of the same metrics object.

### 3. `Mutex`
Only one thread at a time may mutate the shared metrics.

### 4. `Arc<Mutex<T>>`
This is the standard and most important pattern in the project.

### 5. Lock scope
Workers should hold the lock for the shortest time possible.

### 6. Contention
Even with many threads, one shared lock can become the bottleneck.

### 7. `join`
The main thread must wait for every worker before reading final metrics.

### 8. Input ownership vs output sharing
Each worker should own its own input chunk while sharing only the final aggregated state.

---

## Final scope of the project

Keep the project small enough to stay followable, but rich enough to feel real.

### The program should do this

1. Create a list of service events.
2. Split the events into chunks.
3. Create one shared metrics object.
4. Wrap it using `Arc<Mutex<...>>`.
5. Spawn multiple worker threads.
6. Give each worker one owned chunk plus a cloned shared handle.
7. Let each worker process its chunk and update metrics.
8. Wait for all workers using thread joins.
9. Print the final aggregated summary.

---

## What data should exist

### Event
Represents one raw system event.

Recommended fields:

- service
- severity
- status
- message

This keeps the domain concrete and readable.

### Metrics
Represents the final summary.

Recommended tracked data:

- total_events
- total_failures
- critical_events
- counts_by_service
- counts_by_severity

This is enough to make aggregation meaningful without over complicating the design.

---

## Recommended file design

Keep the source footprint intentionally compact.

### `main.rs`
Responsible for orchestration.

It should conceptually own:

- creating sample events
- splitting workload into chunks
- building the shared metrics wrapper
- spawning workers
- joining workers
- printing the final result

### `event.rs`
Responsible for raw event modeling.

It should conceptually own:

- `Event`
- `Service`
- `Severity`
- `Status`

### `metrics.rs`
Responsible for aggregated state and update logic.

It should conceptually own:

- `Metrics`
- how one event updates the summary
- how final reporting is produced

### Optional file if you want slightly more separation
`worker.rs`

Only create this if you feel `main.rs` becomes too orchestration-heavy.
Otherwise keep it at three source files.

---

## Why this file design is good

It creates a very clean separation:

- event modeling is isolated
- metrics logic is isolated
- thread orchestration is isolated

This is interview-friendly and keeps the project easy to reason about.

---

## Systems thinking this project teaches

This project is not just about Rust syntax.
It quietly teaches useful systems ideas.

### Partition the input
Work is divided cleanly into chunks.

### Share only what must be shared
Only the metrics object is shared.
The input chunks are owned independently.

### Minimize the critical section
Shared state should be locked only during actual mutation.

### Separate ingestion from aggregation
Events are the ingestion side. Metrics is the aggregated output side.

### Correctness first, then scaling
A simple global mutex design is right for learning. Performance refinements can come later.

---

## Expected runtime story

When the program runs, different workers may process events in different orders across runs.
That nondeterminism is normal.

But the final aggregated summary should still be logically correct every time.

That distinction matters:

- execution order is nondeterministic
- final result should be deterministic

This is one of the most important concurrency lessons in the project.

---

## What makes this an intermediate project

This project becomes intermediate because it forces you to reason about several ideas at once:

- ownership moving into threads
- thread-safe shared ownership
- synchronized mutation
- scope-based locking
- runtime coordination
- aggregation design

It is not advanced in terms of code volume.
It is intermediate in terms of mental model quality.

---

## Recommended build sequence

When you implement it, build in this order.

### Stage 1: Model the domain
Define the event types and decide what metrics matter.

### Stage 2: Build the metrics logic
Think carefully about how one event changes the summary.

### Stage 3: Build the shared wrapper
Wrap the metrics using `Arc<Mutex<...>>` and understand why both are needed.

### Stage 4: Partition input
Split the full event list into worker chunks.

### Stage 5: Spawn workers
Move owned chunks and cloned shared handles into each worker.

### Stage 6: Join workers
Wait for all threads before reading final metrics.

### Stage 7: Review the lock scope
Check whether the lock is being held only for the update itself.

---

## Common mistakes this project should help you avoid

- thinking `Arc` alone makes mutation safe
- thinking `Mutex` alone solves shared ownership
- locking too early and unlocking too late
- mixing metrics update logic into thread orchestration code
- treating thread execution order as something that should be identical every run
- sharing more state than necessary
- making the event model too vague

---

## Stretch direction after the core version works

Once the simple version is done, the next conceptual improvement is:

Each worker keeps local metrics first, then merges once at the end.

Why that matters:

- much less lock contention
- better scaling
- more realistic optimization path

But that should remain a stretch topic, not the core build for 002.

The core version should focus on mastering `Arc<Mutex<T>>` clearly.

---

## What this project prepares you for next

After this project, you are well-positioned for more serious concurrency topics such as:

- bounded work queues
- multiple producers and multiple consumers
- thread pools
- local aggregation followed by merge
- read-heavy concurrency with `RwLock`
- backpressure and throughput thinking

So 002 is a strong bridge between beginner Rust and more systems-style Rust.

---

## Final recommendation

Project 002 should be rebuilt as:

### `002_concurrent_metrics_aggregator`
A mini project where multiple worker threads process service events and safely update one shared metrics summary using `Arc<Mutex<Metrics>>`.

This is the right next step after Project 001 because it teaches the second major concurrency model in Rust, stays compact enough to build by hand, and still feels practical, interview-worthy, and systems-flavored.
