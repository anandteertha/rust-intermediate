# Theory — Concurrency Pitfalls

## Memory safety and progress

Rust prevents unsynchronized access to ordinary shared mutable memory in safe code. It does not prove that threads finish, that transactions preserve application invariants, or that a particular worker gets a fair share of execution.

An `Arc<Account>` shares ownership. A mutex protects the balance. Neither alone makes a two-account transfer atomic: both balances must stay locked throughout the funds check, overflow check, debit, and credit. Unlocking between those steps could allow stale checks or partial transactions.

## The failing interleaving

Suppose a transfer locks its source first:

| Step | Thread A: account 1 → 2 | Thread B: account 2 → 1 |
| --- | --- | --- |
| 1 | Holds account 1 | Holds account 2 |
| 2 | Needs account 2 | Needs account 1 |
| 3 | Cannot proceed until B releases | Cannot proceed until A releases |

With blocking second acquisitions, this is deadlock. Mutual exclusion, holding one resource while waiting for another, lack of forced resource removal, and circular wait all apply. Rust's mutex guard can correctly release on scope exit while neither thread ever reaches that exit.

The probe forces this state with barriers and observes it using nonblocking `try_lock`. It terminates after reporting the hazard. There is no sleep-based assumption about which worker starts first. The second barrier matters: without it one worker could release its first lock before the other probes, making the observation inconsistent.

## Global lock ordering

Both directions lock the smaller account ID first. One worker can wait for account 1 while the other holds it and obtains account 2. No worker holds account 2 while waiting for account 1. The circular-wait edge is eliminated.

Ordering must be global: every code path acquiring these locks must follow it. An unrelated reverse-order path could reintroduce deadlock. With more accounts, sort all required IDs and acquire in that order. Reject repeated accounts or otherwise deduplicate them before locking a non-reentrant mutex twice.

The account IDs do not determine the direction of payment. After acquiring guards in sorted order, `transfer` maps them back to source and destination. Tests verify both directions so an ordering fix cannot silently reverse transactions.

## Livelock and backoff

A tempting recovery is to acquire the first lock, try the second, release the first on failure, and immediately retry. Both workers can repeatedly collide. They execute and release resources but make no useful progress: livelock.

Randomized backoff can reduce repeated collisions but gives no bounded completion guarantee. Releasing a guard voluntarily is not forcibly preempting another thread's mutex. This project chooses blocking acquisition in a consistent order, avoiding a retry policy altogether.

## Starvation and priority inversion

Starvation means one participant repeatedly misses access while others progress. Removing circular wait does not establish fairness; standard mutex acquisition should not be treated as a FIFO scheduling policy.

Priority inversion is a different dependency: a high-priority thread needs a lock held by a low-priority thread, while medium-priority work prevents the holder from running and releasing it. Priority inheritance is an OS/runtime mechanism that can address this under suitable scheduling policies. This program neither sets priorities nor measures inversion. Keep advanced scheduling protocols separate from this chapter's lock-ordering implementation.

## Lock scope and granularity

One global ledger mutex simplifies transactions but serializes all account operations. Per-account locks permit disjoint transfers to proceed independently but require a consistent multi-lock protocol. Here, every transfer touches the same two accounts, so it still serializes; no throughput improvement is claimed.

Keep logging, sleeping, and unrelated computation outside transaction guards. Keep all balance validation and mutation inside. A short critical section must still include everything required for correctness.

The probe deliberately waits at barriers while holding locks to force an observable hazard. That instrumentation is not part of the transfer algorithm. Inserting the same two-party barrier between the first and second ordered locks would itself hang: the second worker could never obtain the first lock to reach the barrier.

## Validation and limits

Each account initially holds enough to fund all its outgoing work even if the other worker runs later. Each successful operation preserves the mathematical sum and cannot underflow or overflow. Both workers complete 10,000 opposite transfers, so final balances must each be 10,000.

The runner joins every worker before reading final balances and reports worker errors or panics. Tests exercise a forced hazardous interleaving and transaction invariants. Finite passing tests alone do not prove freedom from every deadlock; the lock-order argument supplies the structural reasoning. Process/thread creation failures and unexpected panics in the diagnostic probe are outside the recovery model.
