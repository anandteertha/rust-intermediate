# Rust Intermediate Project 002 - Theory Guide

## Project name
Concurrent Metrics Aggregator

## What this project teaches

This project teaches how Rust handles shared-state concurrency safely.

The central question is:

How can multiple threads update one shared data structure without causing race conditions?

The answer in this project revolves around four major ideas:

- ownership across thread boundaries
- shared ownership with `Arc`
- synchronized mutation with `Mutex`
- structured coordination using thread joins

---

## The two big concurrency models in Rust

Rust programs usually coordinate threads in one of two broad ways.

### 1. Message passing
One thread sends data to another thread, usually through a channel.

Mental model:
I finished my work. Here is the result.

### 2. Shared state
Multiple threads access the same shared value.

Mental model:
We all update the same scoreboard.

Project 001 leaned toward message passing.
Project 002 is about shared state.

That distinction is very important because these are the two major ways concurrent systems are often designed.

---

## Why shared mutable state is dangerous

Suppose two threads both try to increment the same counter.

An increment is usually not one single magical action. It often behaves conceptually like this:

1. read current value
2. add one
3. write the new value

If both threads read the same old value before either writes back, one update is lost.

That is a race condition.

In many languages, this kind of bug is easy to write accidentally.
Rust tries to stop it before the program runs.

---

## Why Rust is strict with threads

Rust already has a core borrowing rule:

- one mutable reference at a time
- or many immutable references
- but not both together

This rule is already useful in single-threaded code.

In multithreaded code, the danger is even bigger.
If many threads could freely hold mutable access to the same data, race conditions would be common.

So Rust forces you to explicitly model:

- who owns the data
- how it is shared
- how mutation is synchronized
- how long the data stays valid

That is why Rust feels strict. It wants concurrency to be intentional.

---

## Ownership and thread spawning

When you spawn a thread, that thread may outlive the current scope.

Because of that, Rust usually wants the new thread to own the things it uses.

That is why thread closures often use `move`.

`move` means:
Take ownership of the captured values and move them into the new thread.

In this project, each worker should usually take ownership of:

- its own chunk of events
- one clone of the shared metrics handle

This is a very clean design because input ownership stays separate while only the final metrics state is shared.

---

## `Rc<T>` vs `Arc<T>`

This distinction is essential.

### `Rc<T>`
`Rc` means reference counted.
It allows multiple owners of the same value.

But it is for single-threaded code only.

Why?
Because its reference count updates are not thread-safe.

### `Arc<T>`
`Arc` means atomically reference counted.
It also allows multiple owners of the same value, but in a thread-safe way.

The word atomic matters because different threads may clone and drop the shared handle safely.

### Important conclusion

- `Rc<T>` is not for cross-thread sharing
- `Arc<T>` is for cross-thread shared ownership

---

## Why `Arc` alone is not enough

Suppose several threads all hold `Arc<Metrics>`.

Now they all share ownership of the same `Metrics` object.
That part is solved.

But if they all try to mutate `Metrics` directly, you still have a concurrency problem.

So `Arc` solves:

- shared ownership across threads

But it does not solve:

- safe concurrent mutation

That is why the project also needs `Mutex`.

---

## What a `Mutex` does

`Mutex` stands for mutual exclusion.

It ensures that only one thread at a time can access the protected data mutably.

Mental model:

There is one room containing the shared data.
Only one thread at a time may enter the room and write on the board.
Everyone else must wait.

So `Mutex<T>` means:

This value can be mutated, but only one thread at a time may do so.

---

## Why `Arc<Mutex<T>>` is the key pattern

This project depends on one very common Rust concurrency pattern:

`Arc<Mutex<T>>`

Each part solves a different problem.

### `Arc`
Lets many threads own the same shared object.

### `Mutex`
Ensures only one thread at a time mutates that object.

So together, `Arc<Mutex<Metrics>>` means:

- many worker threads can all reach the same metrics object
- when a worker needs to update it, it must lock the mutex
- only one worker updates the inner state at a time

A very strong interview sentence is:

`Arc` gives thread-safe shared ownership, while `Mutex` gives synchronized mutable access to the inner data.

---

## Interior mutability in this context

Normally, Rust expects mutable access when you want to mutate something.

With `Mutex`, the outer shared handle can remain shared, while the mutation is allowed through the lock.

That is why this is considered a controlled form of interior mutability.

The safety does not come from magical trust.
It comes from runtime locking guarantees.

---

## Locking and the lock guard

When you lock a mutex, Rust gives you a guard.

That guard means:

- you currently hold the lock
- you may safely access the inner value

When the guard goes out of scope, the lock is released automatically.

This is important because it ties lock lifetime to scope.

---

## Why lock scope matters

If a thread holds the lock longer than necessary, other threads wait longer than necessary.

That reduces concurrency.

Good pattern:

- inspect the event
- prepare what you need
- lock the metrics
- update the shared state quickly
- release the lock

Bad pattern:

- lock early
- do unrelated work while still holding the lock
- finally update the metrics
- unlock late

The shorter the critical section, the better.

---

## What contention means

If many threads want the same mutex, they must take turns.

That waiting is called contention.

This is one of the most important lessons in Project 002:

Threads can exist in parallel, but access to one shared lock is serialized.

That means concurrency does not automatically equal perfect speedup.

For this project, that is okay.
The purpose is to learn safe shared-state design first.

---

## Determinism vs nondeterminism

The order in which threads run may vary from one execution to another.
That is normal.

So execution schedule is often nondeterministic.

But if your locking and aggregation logic are correct, the final metrics summary should still be correct every run.

So the important distinction is:

- thread interleaving may vary
- final logical result should remain correct

That is a foundational concurrency idea.

---

## The domain model in this project

The project should have two major layers of data.

### Raw input: `Event`
Each event represents one service occurrence.

Examples:

- Auth failed login
- Payments timeout
- Cache miss
- Database reconnect success

Useful event fields:

- service
- severity
- status
- message

### Aggregated output: `Metrics`
This is the summary built from all events.

Useful tracked data:

- total events
- total failures
- critical event count
- counts by service
- counts by severity

This separation is good system design:

- events are the workload
- metrics are the derived summary

---

## The high-level runtime flow

The program should conceptually work like this:

1. prepare a list of events
2. split them into chunks
3. create one shared `Metrics` object
4. wrap it in `Arc<Mutex<...>>`
5. spawn workers
6. move one event chunk and one shared handle clone into each worker
7. let each worker process its chunk and update the shared metrics
8. join all workers
9. print the final summary

This is a very clean shape for the project.

---

## Why joining threads matters

Spawning threads is not enough.

If the main thread exits too early, the workers may not finish.

That is why you keep thread handles and call `join` on them.

`join` means:
Wait until this thread finishes.

In this project, joins are necessary because the final metrics should only be printed after all workers are done.

---

## `Send` and `Sync` at a conceptual level

These are foundational Rust concurrency traits.
You do not have to implement them yourself here, but you should understand the intuition.

### `Send`
A type is `Send` if it is safe to move ownership of that value to another thread.

### `Sync`
A type is `Sync` if it is safe for multiple threads to hold shared references to it.

A good memory shortcut is:

- `Send` is about moving across threads
- `Sync` is about sharing across threads

These ideas are part of the reason Rust can statically reason about concurrency safety.

---

## Why this project is good systems practice

Even though the program is small, it teaches strong habits.

### Own the input, share only the summary
This reduces unnecessary shared state.

### Keep update logic centralized
The metrics object should know how to update itself from an event.

### Keep the critical section small
That improves correctness clarity and concurrency behavior.

### Use a simple design first
One shared mutex is the right learning choice here.
Optimization can come later.

---

## Common mistakes to avoid

### Mistake 1
Thinking `Arc` makes mutation safe by itself.

It does not.
It only gives shared ownership.

### Mistake 2
Thinking `Mutex` solves shared ownership by itself.

It does not.
If multiple threads must all own the same protected object, you also need `Arc`.

### Mistake 3
Holding the lock while doing unrelated work.

That increases contention and makes the design worse.

### Mistake 4
Mixing orchestration logic and metrics update logic together.

That makes the code harder to reason about.

### Mistake 5
Assuming thread execution order should be the same every run.

The schedule may vary, but the final summary should still be correct.

---

## A strong mental model for the whole project

Think of the system like this:

- events are incoming service reports
- workers are analysts processing batches
- metrics is one shared scoreboard
- `Arc` lets all analysts reach the same scoreboard
- `Mutex` ensures only one analyst writes on it at a time
- `join` is the manager waiting until everyone is done before reading the final numbers

If this picture is clear in your head, the implementation becomes much easier.

---

## What advanced idea naturally comes next

The simplest version of this project uses one shared global metrics object.
That is absolutely correct for learning.

The next improvement after that would be:

- each worker aggregates locally first
- workers merge results at the end

Why that matters:

- less lock contention
- better scaling
- better systems thinking

But this should be treated as an extension, not the core of Project 002.

---

## Final takeaway

Project 002 teaches one of the most important mental shifts in Rust:

Rust separates sharing from mutation.

- `Arc` handles safe shared ownership across threads
- `Mutex` handles safe mutable access to the shared value

So the heart of this project is not syntax.
It is learning how to design thread-safe shared state deliberately.

Once that becomes clear, the practical build becomes much easier and much more meaningful.
