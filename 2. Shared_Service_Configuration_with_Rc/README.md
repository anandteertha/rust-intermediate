# Rust Intermediate Project 002 - Shared Service Configuration with Rc

This project teaches one of the most important ideas in Rust after basic ownership:

How do multiple parts of a program all own the same value at the same time?

The answer in this project is `Rc<T>`.

## What this project does
A tiny single-threaded application creates one shared application configuration and gives ownership handles to multiple services.

Example services:
- AuthService
- PaymentsService
- ApiGateway

All of them use the same config.

## What this project teaches
- shared ownership
- why one owner is sometimes not enough
- why `mut` does not solve multiple ownership
- how `Rc<T>` works
- what `Rc::clone()` means
- how to model shared read-only app state

## Key learning point
`Rc<T>` is not about mutation.
It is about multiple owners of the same value.

## Why this project matters
A lot of Rust confusion starts here.

People often think:
- if something is mutable, that should be enough
- if I clone something, maybe I copied the data

This project clears that up.

## Core mental model
- one config object exists
- several services need to keep using it
- each service gets an `Rc<AppConfig>`
- all services share ownership of the same underlying value

## Suggested file structure
- `main.rs`
- `config.rs`

## Suggested build steps
1. create `AppConfig`
2. create service structs
3. wrap config in `Rc<T>`
4. clone the handle for each service
5. let each service print a small summary

## What to observe
When the program runs, every service should be reading from the same config source.

That is the heart of the project.

## Best interview sentence
I built a small Rust project where multiple services shared one application configuration using `Rc<T>`. The main lesson was that `mut` does not solve shared ownership, while `Rc` allows multiple owners of the same heap value in a single-threaded design.
