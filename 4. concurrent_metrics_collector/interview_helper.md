# Interview Helper - Project 004

## One-Line Explanation
This project simulates a concurrent metrics collection system where multiple threads safely update a shared metrics object using synchronized access.

---

## Core Problem
Multiple threads need to modify shared data at the same time.

Without proper control, this leads to:
- race conditions
- inconsistent results
- data corruption

---

## Solution Approach

The solution involves two key ideas:

### 1. Shared Ownership
All worker threads need access to the same metrics object.

### 2. Controlled Mutation
Only one thread should modify the shared data at a time.

---

## How It Works

- A shared metrics object is created
- Each thread receives access to it
- Before updating, a thread gains exclusive access
- After updating, access is released
- The main thread waits for all workers before reading final results

---

## Key Concepts to Mention

### Thread Safety
Ensuring multiple threads do not corrupt shared data.

### Race Condition
Occurs when multiple threads modify shared state simultaneously without coordination.

### Synchronization
A mechanism that ensures only one thread accesses critical data at a time.

### Critical Section
The part of the program where shared data is accessed.

---

## Why This Approach Works

- Guarantees correctness
- Prevents data races
- Ensures consistent final results

---

## Limitations

- Threads must wait for access
- Causes lock contention
- Reduces performance under high load

---

## How to Improve This Design

### 1. Reduce Lock Frequency
Accumulate updates locally before applying them.

### 2. Sharding
Split metrics into multiple independent parts.

### 3. Message Passing
Instead of sharing state, send updates to a central processor.

---

## Strong Interview Insight

A correct concurrent system is not necessarily a scalable system.

---

## Follow-Up Questions You Might Get

### Q: Why not allow multiple threads to update directly?
Because it leads to race conditions and inconsistent data.

### Q: What is the downside of locking?
It introduces waiting and reduces parallel efficiency.

### Q: How would you scale this system?
- batch updates
- shard data
- use message passing instead of shared state

---

## Final Takeaway

This project demonstrates how to safely coordinate multiple threads while highlighting the tradeoff between correctness and performance in concurrent systems.
