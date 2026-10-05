# Project 006 — Theory: Backpressure and Resource Budgets

## 1. Begin with the system you already built

In Project 005, producers sent owned events and one consumer aggregated them. The channel coordinated delivery; producers did not directly mutate the consumer's state. Project 006 retains that ownership model and asks: what happens when submission is faster than processing?

Suppose producers offer 30 jobs per second and a consumer completes 5. With unlimited buffering and sustained constant rates, backlog grows by approximately 25 jobs per second. The program can be free of data races and still accumulate more work than it can process.

A queue separates arrival time from processing time. It absorbs bursts. It does not supply extra compute.

## 2. Follow one bounded-queue trace

Capacity is two. Main is processing A.

| Event                       | Processing | Buffered | Waiting send  |
| --------------------------- | ---------- | -------- | ------------- |
| Send B                      | A          | B        | None          |
| Send C                      | A          | B, C     | None          |
| Attempt D                   | A          | B, C     | D             |
| Receive B after finishing A | B          | C        | D can proceed |
| Accept D                    | B          | C, D     | None          |

Receiving removes a job and frees a slot. Processing completion is not required to free that slot. The consumer holds its active job outside the buffer.

A successful buffered send means the channel accepted the value. It does not guarantee eventual receipt if the receiver is later dropped, and it does not confirm processing completion.

## 3. Backpressure

Backpressure is a downstream constraint that limits upstream progress. `sync_channel` supplies it through blocking sends when the buffer is full.

In this implementation each producer creates one job and sends it before creating the next. Blocking therefore limits further generation by that producer. If producers instead prebuilt huge vectors, the channel would not constrain those allocations. If external requests continued arriving elsewhere, backlog could migrate upstream rather than disappear.

Blocking receive waits for a message or disconnection; blocking send waits for space or receiver disconnection. A blocked thread does not repeatedly poll at application level. It still retains resources such as its stack and pending job. Busy-looping over nonblocking operations is a different design and can consume CPU while idle.

## 4. Throughput, latency, and capacity

Throughput counts completed jobs per unit time. End-to-end latency depends on where the clock starts and can include admission wait, queue wait, and processing time.

Our one consumer sleeps for 200 ms per job, giving a nominal service rate of 5 jobs per second. Fifteen jobs require approximately three seconds of processing sleeps. A larger buffer does not shorten those sleeps. Buffering can improve utilization when arrivals are bursty, but cannot accelerate a consumer already continuously busy.

For a job with `n` jobs ahead and fixed processing time `s`, queue wait is roughly `n × s`, with adjustments for the residual active processing and scheduling. Larger backlogs generally mean longer waiting.

Send timing in the current implementation includes `Job::new`, channel operations, blocking, and scheduling delays. The following print is outside the measured expression, although other threads' logging can still affect scheduling. A long send is consistent with backpressure; it is not a precise measurement of queue occupancy.

## 5. Explain the observed output

In the single-producer run, three submissions succeeded quickly despite capacity two. Main had received the first job, leaving two buffered slots available. It printed later because receiving and printing are distinct operations.

In the multi-producer run, many sends took about 600 ms. Main released roughly one slot every 200 ms, and a producer often waited through several releases while competing with other producers. As producers finished, some waits dropped to approximately 400 ms and then 200 ms.

Do not infer round-robin fairness. Each producer's jobs retain their sequential send order, but ordering across producers depends on their actual concurrent sends. An 'attempting' print does not reserve a place in the channel.

## 6. Thread lifetime and shutdown

`spawn` creates a worker; scheduling determines when it executes. `join` waits for termination and does not start a worker. Joining handles sequentially does not serialize workers that were already spawned.

Main drops its original sender so that worker sender lifetimes can determine channel closure. `drop(tx)` does not wait and does not delete buffered messages. Each worker drops its sender as its closure ends. Receiving continues until both conditions hold: the buffer is empty and no senders remain.

Receiving all messages does not replace joining threads. Joining confirms full termination and reports panics. Conversely, joining workers does not drain their messages.

Joining producers before main receives can deadlock: a full queue blocks producers, and main waits for those producers instead of freeing space. A retained sender can also keep an empty receiver waiting indefinitely.

## 7. The memory question: two jobs is not two gigabytes

`sync_channel::<Job>(2)` measures buffered entries. It does not examine payload size or provide byte-budget accounting.

If every job owns a 1 GB allocation, the following snapshot is possible in this architecture:

| Location                                      | Jobs | Payload memory |
| --------------------------------------------- | ---- | -------------- |
| Channel buffer                                | 2    | 2 GB           |
| Consumer processing                           | 1    | 1 GB           |
| Three blocked producers, one pending job each | 3    | 3 GB           |
| Total payloads in this snapshot               | 6    | 6 GB           |

That excludes stacks, queue metadata, allocator overhead, input storage, and processing workspace. This is a hypothetical payload example; the actual `Job` contains only two `usize` fields.

Under explicit assumptions—capacity K, P producers each retaining at most one pending job, W workers each retaining at most one active job, and exclusive payload allocation bounded by S per job—retained payload memory is at most approximately `(K + P + W) × S`. This is not a bound on total process memory. Additional retained jobs, copies, expanding outputs, and workspace invalidate a simple whole-process estimate. Shared payloads must be counted by actual allocations rather than double-counting references.

`size_of::<Job>()` counts inline representation, not heap storage owned by a hypothetical `Vec` or `String`. For allocated memory, length alone can also undercount reserved capacity.

## 8. How would a real system enforce a memory budget?

Your intuition is right: production admission often needs several limits.

| Limit                        | Purpose                                                       |
| ---------------------------- | ------------------------------------------------------------- |
| Maximum payload per job      | Prevent one oversized request from exhausting the budget      |
| Maximum buffered job count   | Limit entry count and metadata overhead                       |
| Global in-flight byte budget | Account for admitted work across queued and processing stages |
| Worker or concurrency limit  | Bound simultaneous processing and workspace demand            |
| Deadline or rejection policy | Decide what happens when resources are unavailable            |

A conceptual byte-budget design reserves a permit for a job's estimated resource cost before allocating or materializing its large payload. The permit travels with the job and is released when accounted resources are no longer retained, including error paths. For a total in-flight budget, releasing it at dequeue is too early: the payload remains alive during processing.

If the input size is unknown, use bounded streaming, capped chunks, validated metadata, or conservative reservation with controlled adjustment. Allocating arbitrarily first and checking afterward can exceed the budget before admission rejects the job. A job larger than the entire budget needs explicit rejection or chunking; waiting cannot make an impossible reservation fit.

This requires application-level accounting; the standard bounded channel does not implement it. Accurate charging may include workspace and outputs, and reservation across several resources must avoid circular waits. These are adjacent design concepts, not extra implementation requirements for Project 006.

## 9. Does capacity depend on hardware?

Yes, but hardware alone does not determine the right queue size. Relevant factors include available memory allocated to this service, payload distribution, processing workspace, measured service rate, burst sizes, latency targets, and overload policy.

More RAM permits a larger backlog; it does not make the backlog finish faster. More cores help only if the application can use them; this program still has one consumer. The 200 ms sleep simulates delay and does not benchmark CPU capabilities.

An illustrative budget: suppose 100 MiB is available for retained payloads after other memory costs have been separately reserved. If every payload is bounded by 1 MiB, three producers each hold at most one pending payload, and one consumer holds one active payload, then `K + 3 + 1 <= 100` suggests K at most 96 from payload memory alone. Metadata and safety margin require a lower practical value.

But with 200 ms service time, dozens of queued jobs imply many seconds of waiting. A latency target may require a much smaller capacity than RAM permits. Measure representative workloads, enforce a budget, and choose a queue size consistent with both memory and response-time goals.

Useful operational measurements include admitted and completed rates, payload bytes retained, processing duration, admission wait, queue wait, and rejected requests. They support tuning but do not enforce a limit by themselves. This simulator records send duration and logs job completion; it does not implement all those metrics.

## 10. Relationship to shared mutable state

A mutex and a channel both involve coordination. One state-owning consumer can become a bottleneck just as one contended mutex can. Message passing simplifies ownership; it does not guarantee scalability.

Local aggregation, smaller critical sections, batching, and partitioned ownership can reduce serialized work. A bounded queue addresses admission and accumulation. A worker pool addresses processing concurrency. They solve different problems.

## 11. Check your understanding

1. Why can a capacity-two channel accept three jobs before processing finishes?
2. Why can a consumer's 'started' log precede a producer's 'submitted' log?
3. Why does dropping every sender preserve pending messages?
4. Why does increasing capacity to 15 not shorten the total processing sleep?
5. Where can memory live outside a bounded queue?
6. Why must an in-flight payload permit survive dequeue?
7. Why might RAM allow capacity 96 while latency goals demand a much smaller value?

Answers and additional curveballs are in `interview.md`. The implementation is complete; these questions consolidate reasoning, not expand scope.

## Official references

- [sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html)
- [SyncSender: send and try_send](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html)
- [Receiver: recv and recv_timeout](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
- [thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [JoinHandle::join](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
- [size_of](https://doc.rust-lang.org/std/mem/fn.size_of.html)

Rust API behavior follows these references. Memory-budget examples and performance calculations are reasoning models with assumptions stated above.
