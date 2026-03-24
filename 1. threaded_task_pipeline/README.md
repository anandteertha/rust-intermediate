# 001_threaded_task_pipeline

A small Rust intermediate project that introduces threads, message passing, and task processing using a worker thread and a channel.

This project reads tasks from a text file, converts each line into a `Task`, sends those tasks from the main thread to a worker thread, and prints a summary for each task as the worker receives it.

## Why this project matters

This is one of the cleanest ways to move from Rust basics into Rust intermediate.

Instead of calling functions directly in a single flow, this project starts thinking in a more systems-oriented way:

- one side produces work
- another side consumes work
- both sides communicate through a channel
- ownership moves safely between threads

## Concepts covered

- `std::thread::spawn`
- `std::sync::mpsc::channel`
- producer-consumer model
- ownership transfer across threads
- graceful shutdown using `drop(tx)`
- parsing text input into domain structs
- basic enum usage with `Priority`

## Project flow

1. The program creates a channel.
2. A worker thread is spawned and waits for incoming tasks.
3. The main thread loads task data from a text file.
4. Each line is converted into a `Task`.
5. The main thread sends each `Task` through the channel.
6. The worker receives tasks one by one and prints their summaries.
7. Once all tasks are sent, the sender is dropped.
8. The worker detects channel closure and exits.
9. The main thread waits for the worker using `join()`.

## Input format

The project reads tasks from a text file.

Each line follows this format:

`id|description|duration|priority`

Example:

`1|Parse config file|2|high`  
`2|Validate user input|1|medium`  
`3|Generate report|3|low`  
`4|Archive logs|2|medium`  
`5|Send alert email|1|high`

## File structure

`src/`  
`main.rs`  
`file_helper.rs`  
`task.rs`  
`priority.rs`  
`sample_tasks.txt`

## Module responsibilities

### `main.rs`

- creates the channel
- spawns the worker thread
- loads the task file
- creates tasks from each line
- sends tasks to the worker
- closes the sender
- waits for the worker thread to finish

### `file_helper.rs`

- loads file content as text

### `task.rs`

- defines the `Task` struct
- parses raw task text into a `Task`
- prints task summaries

### `priority.rs`

- defines the `Priority` enum
- converts raw text like `high` or `low` into enum variants

## How to run

Run the project with `cargo run`.

Make sure the file path inside `main.rs` points to your actual `sample_tasks.txt` file.

## Example output

`High: 1-Parse config file is valid till 2`  
`Medium: 2-Validate user input is valid till 1`  
`Low: 3-Generate report is valid till 3`  
`Medium: 4-Archive logs is valid till 2`  
`High: 5-Send alert email is valid till 1`

## What this project teaches

This project is small in code size, but strong in concepts.

It helps build intuition for:

- how Rust creates worker threads
- how data moves safely from one thread to another
- why channels are often cleaner than shared mutable state
- how to coordinate producer and consumer logic
- why closing the sender is important for graceful shutdown

## Why use a channel here?

A channel makes the design cleaner than using a shared vector between threads.

With a channel:

- the producer sends tasks
- the worker receives tasks
- ownership moves clearly
- thread communication stays structured

This avoids introducing shared mutable state too early.

## Limitations of the current version

This first version is intentionally small and simple.

It currently:

- uses one worker thread
- processes tasks sequentially
- prints summaries instead of simulating full execution
- assumes input is mostly well-formed

That is fine for project 001. The goal is to learn the concurrency model first.

## Possible next improvements

Some natural upgrades for future versions:

- validate malformed task lines safely
- simulate task execution using sleep
- support multiple worker threads
- return task results back to the main thread
- prioritize tasks based on `Priority`
- improve error handling during parsing

## Takeaway

This project is a compact introduction to Rust concurrency.

It shows how to build a basic worker pipeline where:

- the main thread produces tasks
- a worker thread consumes them
- a channel connects both sides safely and clearly

That makes it a strong first project for the Rust intermediate track.
