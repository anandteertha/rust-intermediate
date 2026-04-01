# Rust Intermediate Project 002 - Overview

## Project Name
Shared Service Configuration with Rc

## Folder Name
002_shared_config_with_rc

## Why this project exists
This project teaches shared ownership in Rust.

Rust usually wants one clear owner for each value. That is a very good default, but real programs sometimes need multiple parts of the same application to use the same data.

This project shows that problem in a simple and realistic way:

- one application configuration
- multiple services
- all services need access to the same config
- we do not want to copy the whole config into each service

That is where `Rc<T>` comes in.

## What this project teaches
- why one owner is not always enough
- why `mut` does not solve shared ownership
- what `Rc<T>` really does
- what `Rc::clone()` actually means
- why this is still a single-threaded design
- how to model shared read-only application state cleanly

## Project story
Imagine a tiny backend app with three components:

- AuthService
- PaymentsService
- ApiGateway

All three need to use the same application configuration.

That config may contain things like:
- environment name
- database URL
- retry limit
- feature flags

Without shared ownership, moving the config into one service means the others cannot also own it.

This project fixes that using `Rc<T>`.

## Main idea
Each service will store an `Rc<AppConfig>`.

That means:
- all services share ownership of one config value
- the config is not deeply copied each time
- the config remains alive until the last owner goes away

## Core learning outcome
By the end of this project, you should be able to say:

`Rc<T>` lets multiple parts of a single-threaded Rust program share ownership of the same heap value.

## What this project is not teaching
This project is not about mutation.
This project is not about threads.
This project is not about locks.

It is only about shared ownership.

That narrow focus is what makes it a good Project 002.
