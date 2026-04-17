# Theory - Project 004

## What This Project Is Really About

This project is about shared mutable state across threads.

Projects 1 through 3 stayed in safer or simpler coordination models:

- message passing
- shared ownership in one thread
- shared mutable state in one thread

Project 4 introduces the next major systems problem:

- what if several threads must update the same in-memory state?

That is why this project uses `Arc<Mutex<T>>`.

## The Systems Problem

Metrics are a classic shared-state workload.

Many workers may need to record:

- requests handled
- errors seen
- retries performed
- jobs completed

Conceptually there is one metrics object for the whole process.  
But several threads need to mutate it.

That creates two requirements at the same time:

- several threads must reach the same value
- only one thread should mutate it at a time

## Why `Arc<T>` Is Needed

Across threads, plain ownership is not enough.

Several worker threads all need access to the same metrics value, so the program needs shared ownership that is safe for multithreading.

That is the role of `Arc<T>`:

- atomic reference counting
- shared ownership across threads

This is the threaded counterpart to `Rc<T>`.

## Why `Mutex<T>` Is Needed

Shared ownership alone is not enough.

If multiple threads mutate the same metrics object without coordination, several bad things can happen:

- lost updates
- inconsistent state
- undefined concurrent behavior

`Mutex<T>` creates a critical section:

- one thread locks
- one thread mutates
- others wait

That is the core synchronization idea in this project.

## Why `Arc<Mutex<T>>` Is Such A Common Pattern

This composition is common because each part solves a different problem:

- `Arc` solves shared ownership
- `Mutex` solves synchronized mutation

Together they model:

- many threads can reach the data
- one thread mutates at a time

That makes the design correct, even if it is not the most scalable one.

## Correctness Before Throughput

This project teaches a very important systems lesson:

**a correct concurrent design is the first goal**

Before optimizing for throughput, you must guarantee:

- no data races
- no lost writes
- a valid final state

This project intentionally chooses a correctness-first architecture.

That is the right teaching choice and often the right engineering choice early in a system.

## The Cost Of Centralized Locking

The main tradeoff is contention.

All workers serialize through one lock when they update metrics.

That means:

- correctness is strong
- parallelism is reduced
- throughput may flatten as more workers are added

This is why the project is so good for interview discussion.  
It clearly separates:

- safe concurrency
- scalable concurrency

Those are not the same thing.

## Critical Sections And Their Size

The lock protects the region where shared state is updated.

That protected region is the critical section.

A key systems insight is:

- smaller critical sections usually improve concurrency
- larger critical sections usually increase contention

This project keeps the critical section simple, which makes the pattern easy to see.

## Why Metrics Are A Good Example

Metrics are ideal for learning shared state because:

- they are easy to reason about
- updates are frequent
- correctness is easy to verify
- contention tradeoffs are easy to explain

They also reflect real backend engineering concerns.

In production systems, metrics often become a place where people must decide between:

- simple centralized locking
- atomics
- batching
- sharding
- message passing

## How This Connects To Later Designs

Once you understand this project, you can start asking better architectural questions:

- should each worker batch local counts before locking?
- should counters be sharded by key?
- should simple integer counters become atomics?
- should updates be sent to a dedicated collector thread instead?

That last option leads directly into the message-passing style used in project 5.

## The Bigger Systems Lesson

This project is not only teaching syntax like `lock()` and `Arc::clone()`.

It is teaching that concurrency is fundamentally about coordination under shared resources.

Threads are easy to create.  
Correct coordination is the real challenge.

## Final Mental Model

The best way to think about project 4 is:

- there is one shared metrics object
- several threads need to reach it
- every mutation must be synchronized
- `Arc<Mutex<Metrics>>` is the simplest correct way to express that in Rust

That is the core systems lesson behind the code.
