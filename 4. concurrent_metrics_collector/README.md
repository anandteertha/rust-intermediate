# Project 004: Concurrent Metrics Collector

## Overview

This project shows how to share mutable state safely across multiple threads in Rust.

Three worker threads update one shared `Metrics` value, while the main thread waits for them to finish and then prints the final totals.

It is a small project, but it covers an important systems idea:

**concurrency needs both shared ownership and synchronized mutation**

## What This Project Demonstrates

- spawning multiple threads with `std::thread::spawn`
- sharing one value across threads with `Arc<T>`
- protecting shared mutable state with `Mutex<T>`
- waiting for worker completion with `join()`
- aggregating results after concurrent work finishes

## Why This Matters

In backend and systems code, multiple workers often need to update shared counters such as:

- requests handled
- errors seen
- jobs completed
- retries performed

If multiple threads write to the same data without coordination, the result can become inconsistent.

This project uses `Arc<Mutex<Metrics>>` to make that shared state safe.

## Project Structure

```text
4. concurrent_metrics_collector/
|- Cargo.toml
|- README.md
`- src/
   |- main.rs
   `- metrics.rs
```

## How The Program Works

### `src/metrics.rs`

Defines a simple `Metrics` struct with:

- `requests`
- `errors`

It also provides methods to:

- create a new metrics store
- increment requests
- increment errors
- print the final summary

### `src/main.rs`

The main flow is:

1. Create a shared `Metrics` value wrapped in `Arc<Mutex<_>>`
2. Clone the `Arc` for each worker thread
3. Lock the mutex inside each thread before updating metrics
4. Join all worker threads
5. Lock the metrics one last time and print the final summary

## Concurrency Model

The design can be thought of like this:

```text
Worker Threads -> Arc shared ownership -> Mutex protected access -> Shared Metrics
```

Each worker can reach the same metrics object, but only one worker can mutate it at a time.

That gives us correctness, at the cost of some contention.

## Current Worker Activity

The program currently spawns three workers:

- Worker A adds 3 requests and 2 errors
- Worker B adds 3 requests and 0 errors
- Worker C adds 2 requests and 1 error

Final totals:

- Requests: `8`
- Errors: `3`

## Example Output

```text
Total requests handled:8
Total errors: 3
```

## Run The Project

From this project directory:

```bash
cargo run
```

## Key Rust Concepts

### `Arc<T>`

`Arc<T>` allows multiple threads to own the same value safely.

Without `Arc`, a value cannot simply be shared across thread boundaries by cloning plain ownership.

### `Mutex<T>`

`Mutex<T>` ensures only one thread can mutate the shared data at a time.

That protected section is the critical section.

### `Arc<Mutex<T>>`

This is a common Rust pattern for:

- shared ownership
- thread-safe mutation

`Arc` solves ownership sharing.  
`Mutex` solves synchronized access.

## Tradeoffs

This design is correct and easy to understand, but it does have limits:

- all updates go through one lock
- threads may block waiting for access
- lock contention grows as concurrency increases

So this is a good correctness-first design, but not the most scalable one.

## Possible Improvements

- batch updates locally inside each thread before locking
- shard metrics into multiple independent counters
- use atomic counters for simple numeric metrics
- move to message passing so workers send updates to a collector thread

## Interview Takeaway

This is a strong example for explaining the difference between:

- making concurrent code correct
- making concurrent code scalable

The important insight is:

**a thread-safe design is not automatically a high-throughput design**

## Summary

This project is a compact introduction to shared mutable state in multithreaded Rust.

If you understand why `Arc<Mutex<Metrics>>` is used here, you are already building the right intuition for safer concurrent systems in Rust.
