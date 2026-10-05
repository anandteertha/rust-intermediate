# Project 006 — Bounded Job Queue

A Rust concurrency exercise demonstrating backpressure with three producers, one consumer, and a channel that buffers at most two jobs.

## What this project teaches

Project 005 used multiple producers to send events to one consumer. Project 006 adds a capacity limit: when the channel buffer fills, a producer's `send()` waits until space becomes available or the receiver disconnects.

The learning objective is to distinguish admission capacity from processing capacity. A larger queue can let producers submit sooner without making the consumer finish sooner.

## Implemented behavior

| Setting | Value |
| --- | --- |
| Producers | 3 operating-system threads |
| Jobs per producer | 5 |
| Buffered capacity | 2 jobs |
| Consumer | Main thread |
| Simulated processing | 200 ms sleep per job |
| Timing | `Instant` around each send, including job construction |
| Shutdown | Drop main's sender, drain receiver, join producers |

`Job` holds a private `producer_id` and `job_id`, exposes a constructor and getters, and is transferred by value through the channel. The pair of IDs identifies a job.

The sleep models a slow service. This is a coordination simulator, not a CPU performance benchmark.

## Files

- `src/main.rs`: producer spawning, channel lifecycle, send timing, receiving, processing logs, and joins.
- `src/job.rs`: job representation and accessors.
- `theory.md`: concepts, execution traces, memory budgets, and design tradeoffs.
- `interview.md`: concise explanations and reasoning questions with answers.

Place these documentation files in the Cargo project's root beside `Cargo.toml`.

## Run

From the `Bounded_Job_Queue` directory:

```sh
cargo run
```

No dependencies beyond the Rust standard library are needed by the shown implementation.

## Execution and ownership

Main creates `sync_channel::<Job>(2)`, clones a sender for each producer, and moves each clone into its thread. Each producer creates and submits five jobs sequentially. Main drops its original sender and receives jobs while producers run. For every received job it logs start, sleeps, then logs completion.

Once all senders are dropped, main can still receive buffered jobs. When the buffer is empty and no senders remain, receiving returns an error and the loop ends. Main then joins all producer handles. The submitted implementation pops handles from a vector, so it joins in reverse storage order; that does not determine worker execution or completion order.

## Observed results

The user's runs completed all 15 jobs, five per producer. The single-producer run showed early submissions taking microseconds or less, followed by sends around 200 ms. The three-producer runs showed many sends around 600 ms; near the end of one run, waits fell to roughly 400 ms and then 200 ms as fewer producers remained.

These timings are consistent with producers waiting for slots released approximately every 200 ms. They are observations, not guaranteed fairness or latency bounds. Thread scheduling, logging, and sleep overshoot affect measurements.

A received job is outside the queue. Therefore three jobs can be accepted quickly even with capacity two: one is held by the consumer and two are buffered. Logs may appear in surprising order because a receive and its following print are separate operations.

The expected processing duration is approximately `15 × 200 ms = 3 seconds`, plus overhead. Total runtime was not measured by the current code. Compilation time printed by Cargo is unrelated to job-processing time.

## Scope and limitations

This chapter's core implementation is complete. A completion counter, capacity sweep, structured results, and automated benchmarks are optional extensions, not graduation requirements.

The code uses `unwrap()` for sends and joins, so errors produce panics. It has no persistent storage, cancellation protocol, crash recovery, or fairness guarantee. A successful buffered send is not proof of processing completion. Capacity limits job count in the channel, not total process memory. The present jobs contain only two integers; variable payloads and byte-budget admission are discussed in the theory guide, not implemented here.

## Reading

- [Theory](theory.md)
- [Interview preparation](interview.md)
- [Rust: sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html)
- [Rust: SyncSender](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html)
- [Rust: Receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
- [Rust: JoinHandle](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
