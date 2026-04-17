# Theory - Project 005

## What This Project Is Really About

This project is about choosing message passing over shared state.

Project 4 asked:

- how do multiple threads safely update one shared object?

Project 5 asks a better architectural question for this scenario:

- do they need to update shared state directly at all?

In this project, the answer is no.

The producers only need to report what happened.  
That makes message passing the cleaner coordination model.

## The Systems Problem

Several services generate activity:

- login events
- payment events
- notification events

Those events must eventually become a summary:

- total events
- total failures
- counts per producer
- counts per event type

There are two ways to design this:

- let every producer mutate a shared summary
- let every producer send events to one aggregator

This project chooses the second design.

## Why Message Passing Fits Better Here

If producers only need to report facts, shared locking is often unnecessary.

A message-passing design gives:

- cleaner ownership transfer
- clearer separation of responsibilities
- no lock contention on the summary itself
- one obvious place where aggregation logic lives

That is a strong systems design choice, not just a Rust trick.

## The Pattern Used Here

This project uses a classic pattern:

- multiple producers
- one consumer
- centralized aggregation

Each worker thread produces `Event` values.  
The receiver owns the job of:

- collecting them
- classifying them
- updating summary state

This is a common event pipeline shape in backend systems.

## Why `mpsc` Fits

Rust's standard `mpsc` channel means:

- multi-producer
- single-consumer

That matches the architecture directly:

- several sender clones exist
- one receiver drains the stream

This is why the project feels so natural with channels.

## Why There Is No `Arc<Mutex<EventSummary>>`

That omission is the whole point.

If the summary were shared behind `Arc<Mutex<_>>`, then:

- every producer would lock shared state
- aggregation logic would be spread across many threads
- contention would grow as producer count grows

That would be correct, but less clean for this workload.

In the current design:

- producers create events
- one consumer owns aggregation

That separation makes the system easier to reason about.

## Ownership As Protocol

One of the best systems lessons in this project is shutdown behavior.

The receive loop keeps running while at least one sender still exists.

Once all senders are dropped:

- the channel closes
- `recv()` stops succeeding
- the aggregation loop exits naturally

This means ownership is doing double duty:

- it controls memory safety
- it also defines the lifetime of the event stream

That is a powerful design pattern.

## Event-Driven Thinking

This project helps shift the mental model from:

- "who owns the mutable counter?"

to:

- "what event happened?"
- "who should process that event?"
- "where should aggregation occur?"

That is how a lot of real systems are designed.

Instead of many components reaching into shared state, components emit facts and downstream logic reacts to them.

## Aggregation As A Separate Responsibility

The project also teaches a nice architectural boundary:

- event production is one responsibility
- event summarization is another responsibility

That separation matters because it keeps the system composable.

You can imagine future extensions such as:

- logging raw events
- writing events to a file
- computing rolling summaries
- forwarding events elsewhere

All of those become easier when event generation and aggregation are decoupled.

## The Role Of `HashMap`

`EventSummary` uses `HashMap` to group counts by:

- producer
- event type

That introduces a stream-processing idea:

- receive records
- classify them
- aggregate by key

This is a small but real systems data-processing pattern.

## Important Rust Details Behind The Design

### `move` Closures

Each producer thread uses a `move` closure so it can take ownership of its sender clone.

That makes thread boundaries explicit and safe.

### Moved Events

Each event is moved into the channel on `send()`.

That means there is no shared mutable event object traveling across threads.

### `Eq` And `Hash`

`EventType` derives `Eq` and `Hash` because it is used as a key in `HashMap<EventType, usize>`.

That is a small but important detail behind grouped aggregation.

## Limits Of This Design

This design is clean, but not perfect for every workload.

Current tradeoffs include:

- one receiver can become a bottleneck
- event ordering across producers is nondeterministic
- `std::sync::mpsc` is single-consumer
- the current version is correctness-first, not throughput-first

That is completely fine for this stage of the repository.

## Bigger Systems Ideas This Project Introduces

Even though the program is small, it points toward larger ideas:

- event-driven architecture
- centralized collectors
- decoupling through channels
- lifecycle management through channel closure
- choosing message passing vs locks intentionally

Those are real systems topics, not just language mechanics.

## Final Mental Model

The best way to think about project 5 is:

- producers create facts
- channels transport facts
- one consumer owns interpretation and aggregation

That is the core systems lesson behind the code.
