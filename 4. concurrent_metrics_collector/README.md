# Project 004: Concurrent Metrics Collector

## Overview
This project demonstrates how to safely manage shared mutable state across multiple threads in Rust. It simulates a simple backend system where multiple worker threads update a shared metrics store concurrently.

The focus is on understanding:
- thread creation
- shared ownership
- synchronized mutation
- aggregation of results

---

## Problem Statement
In concurrent systems, multiple workers often need to update shared data such as request counts, error rates, or performance metrics.

Without proper coordination, this leads to:
- race conditions
- inconsistent data
- unpredictable behavior

This project solves that problem using a shared, thread-safe metrics structure.

---

## System Design

The system consists of:

### 1. Shared Metrics
A single metrics object that tracks:
- total requests
- total errors

### 2. Worker Threads
Multiple threads simulate independent workers. Each thread:
- performs some operations
- updates the shared metrics

### 3. Main Thread
The main thread:
- initializes shared state
- spawns workers
- waits for all threads to complete
- prints final aggregated results

---

## Concurrency Model

This project uses:

- Shared ownership across threads
- Controlled access to shared data
- Synchronization via locking

Conceptually:

Threads → Shared Access → Controlled Mutation → Metrics

---

## Execution Flow

1. Create shared metrics
2. Spawn multiple worker threads
3. Each worker:
   - acquires access to shared metrics
   - updates request and error counts
4. Main thread waits for all workers
5. Final summary is printed

---

## Example Output

Total requests handled: 8  
Total errors: 3

---

## Key Learnings

- Multiple threads cannot safely modify shared data without coordination
- Shared ownership allows multiple threads to access the same data
- Synchronization ensures correctness
- Correctness introduces performance tradeoffs

---

## Limitations

- Centralized locking introduces contention
- Only one thread can update metrics at a time
- Not scalable under high concurrency

---

## Future Improvements

- Reduce lock contention using batching
- Partition metrics (sharding)
- Replace shared state with message passing

---

## Summary

This project builds a strong foundation in thread-safe shared state and highlights the tradeoff between correctness and scalability in concurrent systems.