# Theory - Project 001

## What This Project Is Really About

This project is not just about threads.

It is about one of the most important systems design patterns:

- one side produces work
- another side consumes work
- a queue-like boundary sits between them

In this Rust version, that boundary is an `mpsc` channel.

## The Systems Problem

Many programs have two different responsibilities:

- accepting or creating work
- processing that work

If both responsibilities are tightly coupled in one loop, the design becomes harder to evolve.

A pipeline separates those concerns.

The producer decides:

- what work exists
- how work is parsed
- when work is submitted

The consumer decides:

- how work is handled
- how long processing takes
- what happens after receipt

This separation is a systems idea, not just a Rust idea.

## Producer-Consumer As A Coordination Model

This project uses a classic producer-consumer model.

The main thread:

- reads task data from a file
- parses text into `Task`
- sends tasks through a channel

The worker thread:

- waits for incoming tasks
- receives owned `Task` values
- processes them one by one

This gives a useful mental model:

- the producer owns creation
- the consumer owns execution
- the channel owns transit

## Why Message Passing Matters

This project introduces message passing as a way to coordinate concurrent components.

Instead of both threads sharing one mutable vector or list, the producer moves each `Task` into the channel.

That has important effects:

- ownership transfer is explicit
- no shared mutable task buffer is needed
- the worker receives one clear unit of work at a time

This is often simpler than sharing state directly.

## Why The Task Is Parsed Before Sending

The file contains raw text.

The worker does not receive raw lines.  
It receives domain data:

- task id
- description
- duration
- priority

That means the system already shows a useful architecture boundary:

- parse at the edge
- process in the core

This is a common backend design principle.

It keeps the consumer focused on meaningful work rather than input cleanup.

## Graceful Shutdown Through Channel Closure

One of the best lessons in this project is shutdown behavior.

The worker thread keeps receiving until the sender is dropped.

That means:

- there is no separate shutdown flag
- there is no busy waiting loop
- channel closure itself becomes the signal that production is complete

This is an important systems concept:

**resource lifetime can define protocol lifetime**

When the sender no longer exists, the pipeline is over.

## Why `join()` Matters

The main thread does not just fire and forget the worker.

It waits with `join()`.

That matters because a concurrent system is not complete until:

- work submission finishes
- work processing finishes
- the program exits in a known state

Without `join()`, shutdown can become sloppy and timing-dependent.

## Throughput Vs Simplicity

This design is intentionally simple:

- one producer
- one worker
- one channel
- one task at a time

That is not the highest-throughput design, but it is a very strong first architecture.

The project teaches:

- correctness
- clarity
- separation of concerns

before teaching scaling.

## What This Project Hints At Next

Once you understand this project, several larger systems ideas become easier:

- multiple workers
- worker pools
- result channels
- retries
- prioritization
- backpressure

This first version is the foundation for all of those.

## Important Rust Lesson Underneath The Design

Rust makes you feel the difference between:

- owning a value
- borrowing a value
- moving a value

Channels work especially well in Rust because sending usually means moving ownership.

That makes cross-thread coordination safer and easier to reason about.

## Final Mental Model

The best way to think about project 1 is:

- parse work on one side
- send owned work across a boundary
- process work on the other side
- close the channel to end the pipeline cleanly

That is the core systems lesson behind the code.
