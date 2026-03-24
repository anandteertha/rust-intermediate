# Learn Rust Intermediate With Me

[![Language: Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Track: Intermediate](https://img.shields.io/badge/track-rust_intermediate-blue)](.)
[![Style: Project Based](https://img.shields.io/badge/style-project_based-success)](.)
[![Status: Active](https://img.shields.io/badge/status-active-brightgreen)](.)

This repository is the next step after Rust basics.

It is built for learning Rust through small, focused, hands-on projects that introduce more realistic systems concepts without turning into huge codebases.

The goal is simple:

- learn Rust intermediate concepts by building
- keep each project small and easy to understand
- make every project useful for interview prep and learning in public
- build stronger intuition for systems programming in Rust

## Why this repo exists

A lot of Rust learning material jumps from beginner syntax straight into large, intimidating projects.

This repo takes a cleaner path.

Each folder is a compact project that focuses on one important intermediate idea such as:

- threads
- channels
- message passing
- ownership across thread boundaries
- enums and domain modeling
- error handling
- shared state and synchronization
- iterator-heavy data flow
- trait-driven design
- practical systems-style patterns

So instead of learning Rust as isolated theory, you learn it through small working programs.

## What makes this different from the basics repo

The basics repo focused on core language building blocks such as:

- ownership
- borrowing
- structs
- enums
- pattern matching
- traits
- generics
- lifetimes

This intermediate repo keeps the same small-project style, but the concepts go deeper.

Now the focus shifts toward:

- concurrency
- coordination between components
- cleaner program structure
- stronger reasoning about ownership in real systems scenarios
- writing code that feels closer to backend and systems programming

## Repository structure

Each project lives in its own folder.

Current structure:

- `1. threaded_task_pipeline`

More projects will be added over time in the same format.

## Current project

### `1. threaded_task_pipeline`

A small producer-consumer style project where:

- the main thread reads tasks from a text file
- tasks are parsed into Rust structs
- tasks are sent through an `mpsc` channel
- a worker thread receives and processes them
- the sender is dropped for graceful shutdown
- the main thread waits for worker completion with `join()`

This project introduces one of the most important intermediate Rust ideas:

**message passing with ownership-safe concurrency**

## What this repo will cover

This repo is designed to gradually cover topics like:

- worker threads
- channels
- `Send` and `Sync`
- graceful shutdown patterns
- `Arc` and `Mutex`
- shared state vs message passing
- iterator pipelines in real programs
- stronger parsing and error handling
- task processing design
- lightweight systems programming patterns
- practical Rust architecture for small services and tools

## Who this repo is for

This repo is useful if you want to:

- move from Rust beginner to Rust intermediate
- understand Rust beyond syntax memorization
- build small projects instead of reading only theory
- prepare for interviews with explainable projects
- learn in public through GitHub, Medium, LinkedIn, or YouTube

## Learning style of this repo

The style of this repo is intentional:

- small projects over big projects
- clear concepts over unnecessary complexity
- realistic examples over toy syntax demos
- strong explanation value for interviews
- easy to turn each project into content and teaching material

That means most projects stay small, usually one to three files, while still teaching deeper ideas.

## How to use this repo

A good way to use this repository is:

1. pick one project folder
2. read the README inside that project
3. run the code
4. understand the core concept
5. explain the project in your own words
6. move to the next project

You can also use each project as:

- a mini interview story
- a Rust revision note
- a blog post idea
- a video topic
- a building block for a bigger future project

## Why Rust for this track

Rust is one of the best languages for learning systems thinking because it forces you to reason clearly about:

- ownership
- lifetimes
- thread safety
- shared state
- correctness at compile time

That makes it especially strong for backend, systems, and performance-oriented engineering.

## Roadmap mindset

This repository is not about rushing through advanced buzzwords.

It is about building a strong mental model step by step.

The plan is to move from:

- small concurrent programs
- to safer coordination patterns
- to richer intermediate Rust design
- and then eventually toward advanced systems topics

## Repo status

This repository is actively being built in public.

More small Rust intermediate projects will be added over time.

## Connect with me

If you are also learning Rust, preparing for interviews, or interested in systems programming, feel free to explore the projects, follow along, and build with me.

## Takeaway

This repo is a project-based Rust intermediate track for people who want to learn deeper concepts without losing clarity.

If Rust basics teaches you the language, this repo is where Rust starts to feel like real systems programming.
