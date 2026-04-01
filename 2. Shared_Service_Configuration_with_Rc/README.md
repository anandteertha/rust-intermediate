# Concurrent Metrics Aggregator

A mini Rust systems project that teaches shared-state concurrency by having multiple worker threads process service events and safely update one shared metrics object.

---

## Why this project exists

This project is the natural next step after a threaded pipeline project.

The previous step shows how work can move through threads.
This step shows how many threads can safely coordinate around one shared state.

That makes this project a strong introduction to one of Rust's most important concurrency patterns:

`Arc<Mutex<T>>`

---

## What the program does

The program simulates service events such as:

- Auth failed login
- Payments timeout
- Database reconnect success
- Cache miss
- API latency warning

Those events are divided into chunks and processed by worker threads.

Each worker owns its chunk of events, but all workers share one final metrics object.
That shared metrics object is updated safely using Rust synchronization primitives.

At the end, the program prints a summary such as:

- total events processed
- total failures
- total critical events
- count by service
- count by severity

---

## What this project teaches

- shared-state concurrency
- why naive shared mutation is unsafe
- thread-safe shared ownership with `Arc`
- synchronized mutation with `Mutex`
- how `Arc<Mutex<T>>` works
- lock scope and contention
- worker thread joins
- the difference between owned input and shared output

---

## Why this is a mini project

This is bigger than a one-counter mutex demo, but still small enough to build by hand.

It feels practical because it models a tiny observability or telemetry-style component.

The project stays compact while still teaching real systems ideas.

---

## Recommended file structure

### `main.rs`
Orchestrates the full runtime flow.

Responsibilities:

- create sample events
- split input into chunks
- create the shared metrics wrapper
- spawn worker threads
- join all worker threads
- print the final summary

### `event.rs`
Models the raw incoming event data.

Recommended domain pieces:

- `Event`
- `Service`
- `Severity`
- `Status`

### `metrics.rs`
Models the aggregated output and update rules.

Responsibilities:

- track totals
- track grouped counts
- update metrics from events
- format final reporting output

### Optional `worker.rs`
Only add this if you want to separate worker behavior from orchestration.
Otherwise keep the project at three files.

---

## High-level architecture

The runtime flow should look like this:

1. create a list of events
2. split the events into chunks
3. build one shared metrics object
4. wrap it in `Arc<Mutex<...>>`
5. spawn multiple workers
6. move one chunk and one cloned metrics handle into each worker
7. let each worker process its chunk and update the shared metrics
8. join all workers
9. print the final metrics summary

---

## Core design principle

Workers should own their input.
Only the final summary should be shared.

That gives you a clean architecture:

- no unnecessary shared data
- easier reasoning
- more realistic systems design

---

## The most important Rust idea in this project

`Arc` and `Mutex` solve different problems.

### `Arc`
Lets many threads share ownership of one value.

### `Mutex`
Lets only one thread at a time mutate the protected value.

Together, they let many workers safely update one shared metrics object.

---

## Locking principle

Keep the lock for the shortest time possible.

That means workers should do as much as possible outside the lock, then:

- lock
- update metrics quickly
- unlock

This helps reduce contention and keeps the design clean.

---

## Why thread order may look random

Different runs may process events in different orders because thread scheduling is nondeterministic.

That is normal.

The important thing is that the final metrics summary should still be correct every time.

---

## Why this project is interview-worthy

This project is strong because it demonstrates:

- ownership reasoning
- concurrency safety
- shared state design
- awareness of contention
- separation of domain modeling, aggregation logic, and orchestration

A good short way to describe it is:

Built a Rust mini project where multiple worker threads processed service events and updated a shared metrics summary using `Arc<Mutex<Metrics>>`, which helped me understand shared ownership, synchronized mutation, and lock scope in concurrent systems.

---

## Good extension after the core version

A strong next improvement is:

Have each worker aggregate locally first, then merge the local results once at the end.

That reduces contention and introduces a more scalable design.

But that should come after the core project is already clear and working.

---

## Final takeaway

This project teaches that safe concurrency in Rust is not about hoping threads behave.
It is about designing ownership, sharing, and mutation rules explicitly.

That is the real value of Project 002.
