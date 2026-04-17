# Project 005: Multi Producer Event Aggregator

## Overview

This project shows how to coordinate multiple producer threads using Rust channels.

Three service threads generate events and send them into one shared `mpsc` channel.  
The main thread acts as the aggregator, receives each event, updates an `EventSummary`, and prints the final totals after all producers are done.

This project is the natural next step after `Arc<Mutex<T>>` because it shows the other major concurrency style in Rust:

**message passing instead of shared mutable state**

## What This Project Demonstrates

- spawning multiple producer threads with `std::thread::spawn`
- cloning a channel sender for each producer
- sending owned data safely across thread boundaries
- aggregating events through one receiver
- counting totals with `HashMap`
- graceful shutdown by dropping all senders

## Why This Project Matters

In real systems, many components produce signals such as:

- logs
- metrics
- notifications
- job status updates
- user activity events

Very often, those producers do not need direct access to shared state.  
They only need to report what happened.

That is where a channel-based design becomes very useful:

- producers stay simple
- data flow is explicit
- aggregation happens in one place
- synchronization logic stays smaller than a lock-heavy design

## Project Structure

```text
5. multi_producer_event_aggregator/
|- Cargo.toml
|- README.md
|- interview_helper.md
|- notes/
|  `- theory.md
`- src/
   |- main.rs
   |- event.rs
   |- event_summary.rs
   `- event_type.rs
```

## Current Modules

### `src/event_type.rs`

Defines the `EventType` enum:

- `LoginSuccess`
- `LoginFailure`
- `PaymentProcessed`
- `PaymentFailed`
- `EmailSent`
- `SmsSent`

It derives:

- `Copy`
- `Clone`
- `Debug`
- `PartialEq`
- `Eq`
- `Hash`

Those trait derives are important because `EventType` is used as a `HashMap` key inside the summary.

### `src/event.rs`

Defines the `Event` struct:

- `event_type: EventType`
- `producer: String`

It also provides helper methods to read the event type and producer name.

### `src/event_summary.rs`

Defines the aggregation state:

- `total_events`
- `total_success`
- `total_failures`
- `per_producer: HashMap<String, usize>`
- `per_event_type: HashMap<EventType, usize>`

Its `add_event()` method updates:

- the global event count
- the success / failure counters
- counts grouped by producer
- counts grouped by event type

### `src/main.rs`

Coordinates the whole flow:

1. creates the channel
2. clones the sender for three producers
3. spawns three worker threads
4. each worker sends a couple of events
5. drops the original sender
6. receives events until the channel closes
7. joins all producer threads
8. prints the final summary

## How The Program Works

The current program models three services:

- `Notification Service`
- `Payment Service`
- `Login Service`

Each service thread sends two events:

- notification sends `EmailSent` and `SmsSent`
- payment sends `PaymentFailed` and `PaymentProcessed`
- login sends `LoginFailure` and `LoginSuccess`

The main thread receives every event and updates the summary.

## Concurrency Model

The flow looks like this:

```text
Producer Thread 1 ----\
Producer Thread 2 ----- > mpsc channel -> main thread receiver -> EventSummary
Producer Thread 3 ----/
```

This design means:

- producers never mutate the summary directly
- producers only send owned `Event` values
- the summary is updated in one place only

That removes the need for `Arc<Mutex<EventSummary>>` in this project.

## Why Channels Fit Better Here

This project is a strong example of when channels are cleaner than shared locking.

If the producers only need to report events, then:

- a channel makes ownership transfer obvious
- the receiver becomes the single aggregation point
- shutdown logic becomes part of the sender lifecycle

With a mutex-based design, each producer would lock shared state and mutate it directly.  
That would still be valid Rust, but it would be a less clean fit for this specific problem.

## Graceful Shutdown Pattern

One of the most important ideas in this project is channel shutdown.

The main thread drops the original `tx`:

```rust
drop(tx);
```

Each producer thread owns one sender clone.  
When a producer thread exits, that clone is dropped automatically.

The receiver loop:

```rust
while let Ok(event) = rx.recv() {
    event_summary.add_event(event);
}
```

keeps running until all senders are gone.

That means shutdown is driven by ownership, not by a manual boolean flag.

## Current Output

Running `cargo run` currently prints:

```text
total events: 6
total failures: 2
total success:2
per producer: {"Notification Service": 2, "Login Service": 2, "Payment Service": 2}
per event: {PaymentFailed: 1, LoginFailure: 1, SmsSent: 1, LoginSuccess: 1, EmailSent: 1, PaymentProcessed: 1}
```

The `HashMap` display order may vary between runs, which is normal.

## How To Run

From this project directory:

```bash
cargo run
```

## Core Rust Concepts In This Project

### `std::sync::mpsc`

Rust's standard channel supports:

- multiple producers
- one consumer

That matches this design exactly.

### `thread::spawn`

Each producer runs concurrently in its own thread.

### Ownership Transfer Across Threads

Each `Event` is moved into the channel when `send()` is called.

That gives:

- no borrowed event data crossing thread boundaries
- no shared mutable event ownership
- a simpler mental model

### `HashMap::entry`

The summary uses `entry(...).and_modify(...).or_insert(...)` to update grouped counts without manual branching.

## Interview Value

This project gives you a strong way to talk about a real engineering choice:

**sometimes the best concurrent design is not shared state with locking, but one-way message passing**

That is a very useful interview insight because it shows you are thinking about coordination models, not only syntax.

## Limitations Of The Current Version

- the receiver is the main thread rather than a dedicated aggregator thread
- the event stream is very small and deterministic
- there is no retry or recovery if `send()` fails
- the output is summary-focused rather than event-by-event logging

Those are fine tradeoffs for a learning project.

## Good Next Improvements

- print each event as it is received before updating the summary
- add `thread::sleep()` inside producers so interleaving becomes more visible
- move the receiver into its own aggregator thread
- create a richer event struct with timestamps or IDs
- add a `print_pretty_summary()` method instead of printing raw `HashMap` debug output
- separate success / failure classification into a helper on `EventType`

## Summary

This project is a compact introduction to multi-producer event processing in Rust.

It teaches how to:

- spawn multiple workers
- send data across threads safely
- aggregate without shared mutable state
- use sender dropping as a shutdown signal

If project 004 taught shared mutable state with `Arc<Mutex<T>>`, project 005 teaches when message passing can be the cleaner systems design.
