# Interview Preparation — Project 008

## Explain the project

I built a two-account transfer simulator. Opposite-direction transfers would deadlock if each worker locked its source first. A barrier-based diagnostic exposes that circular wait through `try_lock` without hanging. The fixed implementation always acquires the smaller account ID first, validates and updates both balances under the guards, joins every worker, and checks transaction counts and final balances.

## Why does safe Rust allow deadlock?

Memory safety and liveness are different properties. The mutex enforces exclusive access, but Rust does not prove that the thread holding a resource will eventually release it.

## Which deadlock condition did you remove?

Circular wait. All acquisitions follow increasing account IDs. A thread cannot hold a higher-ID account and wait for a lower-ID account through this API.

## Why not just use try_lock in a loop?

Repeated immediate retries can cause livelock or starvation. Randomized backoff reduces collisions but does not guarantee completion within a fixed time. Consistent blocking lock order is simpler for this transaction.

## Why do both balances stay locked?

A transfer includes checking available funds, checking destination overflow, and updating both balances. Releasing a lock before finishing would expose an intermediate state or invalidate checks. Error paths validate before changing either balance.

## Does the probe actually deadlock?

No. Each second acquisition is nonblocking. Two barriers force both first locks to remain held during both probes. Blocking those acquisitions instead would make the demonstrated circular wait permanent.

## Is the algorithm fair or faster?

No fairness or speedup is established. Both workers contend on the same account pair. The improvement is a consistent acquisition protocol that removes this deadlock cycle while preserving balances.

## Deadlock, livelock, starvation, priority inversion?

- Deadlock: participants wait in a dependency cycle and cannot proceed.
- Livelock: participants keep reacting or retrying without useful progress.
- Starvation: one participant fails to progress while others continue.
- Priority inversion: a high-priority participant depends on a resource held by a lower-priority participant whose execution can be delayed by intermediate-priority work.

## What would change in a production ledger?

Unique account identity would be enforced centrally. Persistence, transaction isolation, crash recovery, idempotency, and an audit trail would need explicit designs. In-process mutexes and final balance checks alone do not provide those properties.

## Self-check

1. Trace the two-thread circular wait before applying the fix.
2. Explain why reversing payment direction does not reverse acquisition order.
3. Explain why one barrier is insufficient for the diagnostic.
4. Predict what happens if a barrier is inserted after the first ordered acquisition.
5. Explain why matching final balances alone would not prove every operation ran.

Answers are in the theory notes and tests; practice explaining them without opening the source.
