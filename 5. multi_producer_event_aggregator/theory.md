# Theory Notes - Project 005

## What This Project Is Really Teaching

On the surface, this project is about threads and channels.

At a systems level, it is teaching something more important:

**how to choose a coordination model**

There are two common ways concurrent components work together:

- shared state
- message passing

Project 004 focused on shared state with `Arc<Mutex<T>>`.  
Project 005 focuses on message passing with `mpsc`.

Both are valid.  
The real skill is knowing when each one fits.

## Shared State Vs Message Passing

### Shared State

Shared state means multiple threads can access the same data structure.

That usually requires:

- shared ownership such as `Arc<T>`
- synchronized access such as `Mutex<T>`

This works well when several threads truly need to read or mutate the same in-memory state.

But it also introduces:

- lock contention
- critical sections
- more reasoning about who can mutate what and when

### Message Passing

Message passing means one component sends data to another instead of mutating shared state directly.

That usually gives:

- clearer ownership transfer
- fewer shared mutable structures
- more explicit data flow

This works especially well when one side only needs to report events, results, or tasks.

That is exactly the case in this project.

## The Pattern Used Here

This project uses a classic pattern:

- multiple producers
- one consumer
- centralized aggregation

Each producer thread represents a service.

Each service emits `Event` values.

The receiver owns the job of:

- collecting events
- summarizing them
- deciding when processing is complete

This is a very common backend and systems pattern.

## Why `mpsc` Fits

Rust's `std::sync::mpsc` literally means:

- multi-producer
- single-consumer

That matches the project design one-to-one.

Several sender clones can exist at the same time.  
Only one receiver owns the incoming stream.

That receiver becomes the natural place for aggregation logic.

## Ownership As A Shutdown Mechanism

One of the best things about channel-based designs is that shutdown can fall out naturally from ownership rules.

The receive loop continues while at least one sender still exists.

When all senders are dropped:

- the channel closes
- the receiver stops blocking forever
- the loop can exit cleanly

This matters because many concurrent systems fail not during normal processing, but during shutdown and cleanup.

In this project, ownership helps define shutdown behavior clearly.

## Why There Is No `Arc<Mutex<EventSummary>>`

That is a deliberate design choice.

If the summary were wrapped in `Arc<Mutex<_>>`, then:

- every producer would need to lock it
- every producer would own part of the update logic
- contention would grow as producer count grows

That would still be correct, but it would make the mental model noisier.

Instead, this project keeps one thread responsible for aggregation.  
That reduces synchronization complexity and improves readability.

## Event-Driven Thinking

This project also introduces a more event-driven mindset.

Instead of thinking:

- "which thread owns this mutable counter?"

you start thinking:

- "what event happened?"
- "who should consume that event?"
- "where should aggregation happen?"

That shift matters a lot in real systems design.

Many production systems are easier to build and reason about when components communicate by events rather than by directly sharing mutable state.

## The Role Of `HashMap`

The summary uses `HashMap` for grouped counting.

That introduces a common data-processing pattern:

- ingest events
- classify events
- aggregate by key
- print or export results

This is useful well beyond concurrency.  
It is also a small introduction to stream-style processing.

## Important Rust Details Behind The Scenes

### `EventType` As A Hash Key

Because `EventType` is used as a key in `HashMap<EventType, usize>`, it must implement:

- `Eq`
- `Hash`

That is why the enum derives those traits.

### Moved Values

When a producer calls `send(event)`, the event is moved into the channel.

That means:

- the sender no longer owns it
- the receiver becomes the next owner

This is one reason Rust channels feel so safe: ownership transfer is part of the API.

### `move` Closures In Threads

Producer threads use `move` closures so each thread takes ownership of its sender clone.

Without `move`, the closure would try to borrow from `main`, which does not work for spawned threads that may outlive the current scope.

## Limits Of This Design

This project uses a correct and clean model, but every model has tradeoffs.

Current limits include:

- one receiver can become a bottleneck
- standard `mpsc` does not support multiple consumers on the same receiver stream
- event ordering between producers is nondeterministic
- throughput is not optimized

That is fine for a learning project because the goal here is correctness and design clarity first.

## Bigger Systems Ideas This Project Hints At

Even though the code is small, it introduces ideas that scale into bigger systems topics:

- producer-consumer pipelines
- centralized aggregation
- decoupling components through events
- graceful shutdown protocols
- correctness before throughput
- choosing between locks and message passing

Those are real systems engineering topics, not just language exercises.

## Final Mental Model

The best way to think about project 5 is:

- producers create facts
- channels transport facts
- one consumer interprets and aggregates those facts

That is the core systems lesson behind the code.
