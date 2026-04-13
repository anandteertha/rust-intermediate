# Theory - Project 004

## The Core Problem
Concurrency introduces multiple execution flows accessing the same memory.

## What Can Go Wrong
- Data races
- Lost updates
- Inconsistent reads

## Fundamental Requirements
1. Shared ownership across threads
2. Controlled mutation

## Synchronization
Only one thread should modify shared state at a time.

## Tradeoffs
- Strong synchronization ensures correctness
- But reduces parallelism

## Scalability Issue
Centralized locking creates contention.

## Evolution Path
1. Shared state with locks
2. Reduce lock frequency
3. Partition data (sharding)
4. Move to message passing

## Mental Model
Think of concurrency as coordination, not just parallel execution.
