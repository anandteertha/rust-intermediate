# Learn Rust Intermediate With Me

[![Language: Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
![Track: Intermediate](https://img.shields.io/badge/track-rust_intermediate-blue)
![Style: Project Based](https://img.shields.io/badge/style-project_based-success)
![Status: Active](https://img.shields.io/badge/status-active-brightgreen)

This repository is the next step after Rust basics: small, focused projects that teach intermediate Rust alongside the systems concepts behind the code.

Each project introduces a specific problem, builds a working solution, and explains its behavior and tradeoffs. The goal is to understand why a design works—not just which Rust types make it compile.

## Why this repo exists

Moving from ownership exercises to large systems projects can leave a gap in understanding. This track fills that gap one manageable project at a time.

The goals are to:

- Learn intermediate Rust by building and explaining working programs.
- Connect ownership and borrowing to concurrency, coordination, and resource management.
- Keep each implementation small enough to reason about completely.
- Create useful revision notes and interview explanations.
- Build components that can eventually fit into a larger system.

## What changes after Rust basics

The basics track covered ownership, borrowing, structs, enums, pattern matching, traits, generics, and lifetimes.

Here, those building blocks become tools for answering questions such as:

- Who owns data when it moves between threads?
- When should components share state, and when should they send messages?
- What makes a thread wait, and what allows it to resume?
- How do we know that all work has finished?
- What happens when work arrives faster than it can be processed?
- Which operations must be coordinated, and which can run independently?

## Repository structure

Each project lives in its own folder. The completed sequence is:

| Project | Folder | Main concept |
| --- | --- | --- |
| 001 | `1. threaded_task_pipeline` | Owned tasks passed from main to a worker |
| 002 | `2. Shared_Service_Configuration_with_Rc` | Shared ownership in one thread |
| 003 | `3. Shared_draft_editor` | Interior mutability in one thread |
| 004 | `4. concurrent_metrics_collector` | Shared mutable state across threads |
| 005 | `5. multi_producer_event_aggregator` | Multiple producers and centralized aggregation |
| 006 | `6. Bounded_Job_queue` | Bounded buffering and backpressure |
| 007 | `7. worker_pool_and_result_collection` | Persistent workers and result collection |
| 008 | `8. concurrency_pitfalls` | Circular wait, global lock ordering, and transaction invariants |

Project documentation explains the implementation and its systems concepts. Project 006 includes `README.md`, `theory.md`, and `interview.md`.

## Completed projects

### 001 — Threaded Task Pipeline

Main reads tasks from a text file, parses them into Rust structs, and sends owned tasks through an `mpsc` channel. A worker receives and processes them. Dropping the sender allows the worker to finish receiving, and main joins the worker.

**Core idea:** message passing transfers ownership across thread boundaries.

### 002 — Shared Service Configuration with `Rc`

One `AppConfig` is wrapped in `Rc<AppConfig>`. Auth, payment, and API gateway components receive cloned ownership handles and read the same configuration without deep-copying it.

**Core idea:** multiple components can own the same value within a single-threaded program.

### 003 — Shared Draft Editor

A document is wrapped in `Rc<RefCell<Document>>`. Editors update its title and body through `borrow_mut()`, while a reviewer reads it through `borrow()`. Versioning and document statistics make the changes visible.

**Core idea:** interior mutability permits shared mutation with borrowing rules checked at runtime. `Rc<RefCell<T>>` is a single-threaded pattern.

### 004 — Concurrent Metrics Collector

Three workers share `Arc<Mutex<Metrics>>`. Each worker locks the metrics before updating counters. Main joins the workers and prints the final totals.

**Core idea:** `Arc` provides shared ownership across threads, while `Mutex` coordinates exclusive access to mutable state.

The design also raises a scaling question: how much work happens while holding the lock? Shared state is a valid approach; contention grows when many workers need the same lock. Short critical sections, local aggregation, and partitioning can reduce that contention when the application's correctness requirements allow them.

### 005 — Multi-Producer Event Aggregator

Three service threads send owned events through cloned `mpsc` senders. Main receives and aggregates events by producer and event type. Once all senders are dropped, main drains the remaining events, exits the receive loop, and joins the workers.

**Core idea:** one consumer can own aggregation state while other threads communicate through messages.

Workers start through `spawn()`. Calling `join()` later waits for completion; sequential joins do not make already-spawned workers execute sequentially. Events sent before main starts receiving remain buffered.

### 006 — Bounded Job Queue

Three producer threads each submit five jobs through `sync_channel::<Job>(2)`. Main receives one job at a time and simulates processing with a 200 ms sleep. Producers measure their send durations; main logs processing start and completion.

**Core idea:** a full bounded queue makes producers wait, applying backpressure to sequential job generation.

The observed runs completed all 15 jobs. Many sends waited approximately 600 ms while three producers competed for slots; waits decreased as producers finished submitting. These observations illustrate contention, not a guaranteed fairness policy.

This project distinguishes:

- **Buffered jobs from unfinished jobs:** the consumer and blocked producers can hold jobs outside the queue.
- **Submission from completion:** a successful send does not mean processing has finished.
- **Queue capacity from throughput:** more buffering does not speed up an already busy consumer.
- **Job-count limits from memory budgets:** variable payload sizes and allocations outside the queue require separate accounting.

The sleep makes this a coordination simulator, not a CPU benchmark. Approximately three seconds of processing sleep is expected for 15 jobs; total runtime was not measured by the current implementation.

## Shared state and message passing

Projects 004 and 005 explore different ownership and coordination models.

| Question | Shared mutable state | Message passing |
| --- | --- | --- |
| Who accesses the state? | Workers access it through synchronization | An owner receives requests or events |
| How are operations coordinated? | Locks or other synchronization primitives | Channel operations and the owner's processing loop |
| Where can contention appear? | A heavily used lock or memory location | A saturated queue or state-owning consumer |
| How might it scale? | Shorter critical sections, batching, partitioning | Batching, partitioning, additional processing capacity |

Neither approach automatically guarantees scalability. The important question is how much work must happen sequentially and whether independent work can proceed separately.

## Learning workflow

For each project:

1. Understand the motivating problem and predict its behavior.
2. Learn the systems concept and relevant Rust tools.
3. Implement one manageable step at a time.
4. Run it and explain the output, including scheduling surprises.
5. Consolidate the theory and interview explanation.
6. Move on once the learning objective is met.

Optional extensions remain optional. A completed chapter does not need to grow into a large application.

Follow the numbered sequence: it moves from ownership transfer to shared ownership, shared mutation, thread coordination, and bounded admission.

## Planned progression

Projects 007 and 008 are implemented. Project 008 uses a focused account-transfer exercise to demonstrate conflicting lock orders safely and prevent circular wait through global lock ordering. See its `README.md`, `theory.md`, and `interview.md`.

The following projects are planned, not completed:

| Project | Focus | Question |
| --- | --- | --- |
| 009 | Async job runner | How does coordinating asynchronous tasks differ from using threads? |
| 010 | Async job service | How do admission, execution, and lifecycle policies fit together? |

The longer-term direction is a model-execution orchestrator assembled from these smaller lessons. Each chapter remains focused rather than jumping directly to the capstone.

Other intermediate topics can be introduced where they help solve a concrete problem: `Send` and `Sync`, parsing and error handling, trait-based design, iterator pipelines, and resource-aware scheduling.

## Who this repo is for

This track is for learners who know Rust basics and want to build stronger concurrency and systems intuition. The projects also provide explainable examples for interviews, revision, and learning in public.

## Why Rust

Rust makes ownership and access rules explicit. That helps connect program structure to questions about thread safety, lifetimes, shared state, and resource use.

Compile-time safety is an important foundation, but it does not replace systems reasoning: a safe program can still deadlock, accumulate too much work, or have poor throughput. These projects explore both correctness and behavior.

## Status and participation

Projects 001–008 form the completed sequence of implementations. The repository remains active, with more focused projects planned.

Explore the code, run the examples, read the accompanying notes, and explain the tradeoffs in your own words. If you are learning Rust or systems programming too, follow along and build with me.
