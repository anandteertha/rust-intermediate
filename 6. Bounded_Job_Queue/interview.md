# Project 006 — Interview Preparation

## Explain the project in 30 seconds

“I built a bounded job queue in Rust using three producer threads and one consumer on main. Each producer submits five owned jobs through a channel with capacity two. The consumer simulates 200 milliseconds of processing per job. I timed sends and observed producers blocking when the buffer filled. I dropped main's sender, drained the queue, and joined the workers. The exercise taught me that buffering controls accumulation while consumer capacity controls completion throughput.”

## Core questions

### 1. What is backpressure?

A downstream resource limit constrains upstream progress. Here, a full buffer blocks sends, delaying each producer's next job. It is effective across this generation path because producers create and send sequentially.

### 2. How is this different from an unbounded channel?

An unbounded channel does not wait for a buffer slot. With sustained overload, buffered work can keep accumulating. A bounded channel limits stored entries and makes the sender wait when full.

### 3. Does capacity two mean at most two unfinished jobs?

No. The consumer can hold one active job and each producer can hold a pending job while blocked. Capacity counts only buffered jobs.

### 4. Does sending successfully mean processing completed?

No. It means the channel accepted the job. Completion requires separate evidence such as a result or acknowledgment. Even capacity zero confirms handoff, not processing completion.

### 5. When does a producer unblock?

A waiting send can proceed when buffer space becomes available, or it returns an error if the receiver disconnects. Receiving a queued job frees space before processing finishes. No fixed selection order among competing producers should be assumed.

### 6. Why were some sends about 600 ms when processing takes 200 ms?

The producer waited through several slot releases while competing with other producers. That reflects observed scheduling, not a guaranteed formula or fairness policy.

### 7. Why did three sends succeed quickly with capacity two?

Main had already received one job. That job was outside the buffer, allowing two additional jobs to be queued.

### 8. Why can 'started' appear before 'submitted' for the same job?

The send has progressed enough to make the job available, but the producer may not yet execute its following print. The consumer can receive and print first. Log order is not a complete trace of internal channel operations.

### 9. Why call drop(tx)?

Main does not need its sender after spawning workers. Retaining it would keep the channel connected even when workers finish, potentially leaving an empty receive waiting forever. Dropping main's sender neither waits nor removes queued messages.

### 10. When does the receive loop end?

When the buffer is empty and all senders have been dropped, `recv()` returns an error. An empty buffer alone is insufficient if a sender still exists.

### 11. Why join after consuming?

Producers may block on a full queue. If main waits for them before receiving, neither side can progress. Receiving first lets producers finish. Joining afterward checks thread termination and panics.

### 12. Do sequential joins serialize the workers?

No. The workers were started by `spawn`. Main waits for their completion in join order, while workers continue independently. A worker already finished when joined returns immediately.

### 13. Does blocking waste CPU?

Blocking waits do not repeatedly poll at application level. The waiting thread still occupies resources. A tight loop over `try_recv()` or `try_send()` can waste CPU and is a different design.

### 14. Would a bigger queue increase throughput?

Not for a consumer already continuously busy with fixed 200 ms processing. It increases buffering and can reduce producer admission waits. Buffering can help utilization when arrivals are intermittent, but cannot create extra service capacity.

### 15. Why is expected runtime roughly three seconds?

One consumer processes 15 jobs sequentially, sleeping 200 ms for each. Those sleeps total three seconds, plus scheduling and logging overhead. The current implementation does not measure total elapsed time.

### 16. Why is no application mutex needed for completion tracking?

Only main would access the completion state. Producers communicate by transferring jobs. The channel performs its own internal synchronization. Single ownership avoids directly shared mutable application state.

### 17. What does capacity zero mean?

A rendezvous channel: a send waits for a corresponding receive instead of storing a buffered job. It does not eliminate active jobs or payloads held by producers.

### 18. What happens if the receiver is dropped?

Pending or subsequent sends return errors. In this implementation `unwrap()` converts a send error into a producer panic, which a later join reports as an error. Messages left in a dropped receiver's buffer are not delivered.

### 19. What does a receive timeout prove?

Only that an event was not received within the interval. It does not prove the producer is dead, close the channel, or cancel work. The same receiver can be used again.

## Memory and hardware curveballs

### 20. Two queue slots, 1 GB jobs: is process memory bounded to 2 GB?

No. Two buffered jobs, one active job, and three blocked producers each holding a payload can account for 6 GB of payload memory, plus other allocations. This assumes distinct allocations and the hypothetical large-payload version of the program.

### 21. Does size_of::<Job>() measure all payload memory?

Only for inline data does it capture that representation. A `Vec` or `String` owns separate heap storage; its header size does not include that allocation. Reserved capacity and processing workspace also matter.

### 22. How would you enforce a byte budget?

Use application-level admission accounting in addition to count limits. Reserve estimated bytes before creating large allocations, carry the reservation with the job, and release it when those resources are no longer retained. Reject or chunk jobs larger than the entire budget. Account for failures and variable processing workspace.

### 23. Why not release a byte reservation at recv()?

That would bound queued bytes only. The consumer still holds the payload during processing. To bound total in-flight payloads, the reservation must remain held until the accounted payload is released.

### 24. Should the queue inspect the job's size automatically?

This channel treats a job as a typed value and limits entries. It does not know application-specific heap ownership, processing expansion, or resource cost. A wrapper or admission controller must define and enforce those costs.

### 25. Is the correct queue capacity determined by RAM?

RAM constrains feasible capacity, but response-time goals can require a smaller queue. Consider payload sizes, worker workspace, available service memory, arrival bursts, processing rates, and rejection policy. This sleeping simulator does not benchmark the CPU.

### 26. If jobs are partitioned among producers, does that solve overload?

Partitioning can reduce shared-state contention, but does not solve a downstream service-rate mismatch. Independent ownership and bounded admission address different constraints.

## Explain an apparent contradiction

**Question:** “Producer A printed 'attempting' before producer B. Why was B received first?”

**Answer:** Printing an attempt is not enqueueing. A may be descheduled before its send, while B sends first. Across concurrent producers, log order does not establish send order. Within one producer, sequential submissions retain their order.

## Design exercise, with answer

**Scenario:** A queue has capacity two. Three producers each submit a 1 MiB job at a time, one consumer processes one job at a time, and payloads are exclusively owned. You have a 10 MiB payload budget. Is queue capacity enough to enforce that budget?

**Answer:** Under exactly those assumptions, at most six payloads can be retained across buffered, pending, and active jobs, accounting for 6 MiB. However, the queue alone does not enforce the 1 MiB maximum, the producer generation discipline, or the absence of extra allocations. Enforce those assumptions and separately budget stacks, metadata, outputs, and workspace. Changing the workload can invalidate the bound.

## Avoid these claims

- “Bounded means total memory is bounded.” State exactly what is bounded.
- “A successful send proves the job ran.” It proves admission or handoff, not completion.
- “Three producers rotate fairly.” The observed run does not guarantee fairness.
- “Bigger queues make processing faster.” Explain the actual bottleneck.
- “Channels eliminate synchronization.” They encapsulate coordination.
- “Shared state cannot scale.” Contention and serialized work determine the bottleneck.
- “My measurements benchmark CPU speed.” The workload mainly sleeps and prints.

## Chapter status

The core implementation and observed behavior meet Project 006's learning goal. Counters, capacity sweeps, timeouts, byte permits, and worker pools are not required additions to this chapter. The next project can increase processing concurrency with a worker pool and result collection.

## Official references

- [sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html)
- [SyncSender](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html)
- [Receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
- [JoinHandle](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
