# Project 008 — Concurrency Pitfalls and Recovery

One focused project: transfer money between two shared accounts in opposite directions, understand the circular-wait hazard, and prevent it with global lock ordering.

## Prerequisites and objective

Builds on project 007: `thread::spawn`, `Arc`, `Mutex`, guard lifetimes, and joining workers. Theory prerequisites are deadlock, livelock, starvation, contention, and priority inversion. The practical objective is to distinguish memory safety from progress and preserve a transaction invariant under concurrency.

## Specification

- Two accounts have distinct, immutable IDs and independently locked balances.
- Two threads transfer one unit in opposite directions, 10,000 times each.
- Every successful transfer holds both locks across validation and mutation.
- Both directions acquire accounts in ascending ID order.
- Reject insufficient funds, destination overflow, and equal IDs without mutation.
- Join both workers and verify final balances equal the initial balances.
- First demonstrate the conflicting-order hazard without hanging the process.

The hazard probe has two workers acquire different first locks. A barrier ensures both hold their first lock before attempting the other. `try_lock()` reports both second acquisitions as blocked; a second barrier prevents either worker releasing its first lock prematurely. Replacing those probes with blocking `lock()` while retaining the first guards would deadlock. This is a controlled diagnostic, not an actual deadlock or a retry loop.

## Run and check

```powershell
cd "C:\Users\anand\Development\rust-intermediate\8. concurrency_pitfalls"
cargo run
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Expected output:

```text
Conflicting order: 2 second-lock acquisitions would block.
Blocking on those second locks would create circular wait.
Global order: 20000 transfers completed; balances=[10000, 10000]
All workers joined; balances preserved.
```

The tests check the forced conflicting interleaving, both transfer directions, final balances and operation counts, rejected transfers, duplicate IDs, and an empty workload. No scheduling or fairness claim is based on timing measurements.

## Files and boundaries

`src/lib.rs` implements the account transaction, hazard probe, runner, and tests. `src/main.rs` prints the demonstration. Read [theory.md](theory.md) and [interview.md](interview.md) for the reasoning.

The global ID uniqueness rule is a caller contract; this example creates exactly two accounts. Equal IDs are rejected. Balance reads are separate snapshots; the final summary reads after joining, when no writers remain. Poisoned account locks return an error rather than silently recovering potentially inconsistent balances.

This standard-library exercise does not simulate OS priorities, implement randomized backoff, guarantee fairness, or provide a production ledger. The remaining pitfalls inform the design discussion rather than expanding the project into unrelated experiments.
