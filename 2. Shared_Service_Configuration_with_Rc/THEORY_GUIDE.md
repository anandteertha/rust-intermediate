# Rust Intermediate Project 002 - Theory Guide

## Project Name
Shared Service Configuration with Rc

## The real question this project answers
Rust usually wants one owner for each value.

So what do you do when multiple parts of your program all need to keep using the same value?

That is the exact problem `Rc<T>` solves.

## One owner by default
In normal Rust ownership:
- one value has one owner
- moving that value transfers ownership
- after a move, the old owner cannot use it anymore

This is one of Rust's core safety rules.

## Why one owner is sometimes not enough
Many programs have shared read-only data such as:
- config
- metadata
- templates
- lookup tables
- application context

Several parts of the program may all need to keep using the same value.

If one service owns the config, another service cannot also own it.
That is the gap `Rc<T>` fills.

## What `mut` does and does not do
`mut` allows mutation through a binding.

It does not:
- create multiple owners
- relax move rules
- let two structs both own the same value

This is the biggest confusion for many learners.

Important sentence:

`mut` is about mutation.
`Rc<T>` is about shared ownership.

Those are different concepts.

## What `Rc<T>` is
`Rc<T>` stands for reference counted.

It is a smart pointer that lets multiple owners point to the same heap-allocated value.

When you clone an `Rc<T>`:
- Rust does not deep-copy the inner value
- Rust creates another smart pointer to the same allocation
- the reference count increases

When one owner goes away:
- the count decreases

When the last owner goes away:
- the inner value is dropped

## What `Rc::clone()` really means
This is important.

`Rc::clone()` does not mean:
- make a brand new config object
- duplicate all fields deeply

It means:
- create another owner of the same underlying value

So multiple services can all hold their own `Rc<AppConfig>` handles while still sharing one actual config.

## Why `Rc<T>` is useful
You use `Rc<T>` when:
- multiple parts of a single-threaded program need ownership of the same value
- copying the full value is wasteful or conceptually wrong
- the shared value is mostly read-only

This is common in tree-like structures, app context objects, config sharing, and graph-style relationships.

## Why `Rc<T>` is single-threaded
`Rc<T>` is for single-threaded designs.

The reason is simple:
- its reference counting is not designed for cross-thread coordination

So the mental rule is:
- `Rc<T>` is for shared ownership inside one thread of execution

For this project, that is perfect, because we only want to understand shared ownership first.

## Why this project should not include mutation
If we add mutation here, the project starts teaching too many things at once.

Then the learner has to understand:
- shared ownership
- runtime borrow rules
- interior mutability

That belongs in a later project.

Project 002 should stay focused on one thing:
shared ownership with `Rc<T>`.

## The app config story
This project uses one shared application configuration.

That config might hold:
- environment
- database URL
- retry limit
- feature flag state

Three services need access to it:
- AuthService
- PaymentsService
- ApiGateway

That creates the perfect shared ownership scenario.

## What should happen conceptually
- build one config value
- wrap it in `Rc<T>`
- give each service its own cloned handle
- each service reads from the same shared config
- no deep copies are needed

## What the learner should observe
At the end of the project, the learner should clearly understand:
- all services point to the same logical config
- each service has ownership of a handle
- the config is not copied over and over
- the config stays alive as long as at least one owner remains

## Best one-line understanding
`Rc<T>` allows multiple owners of the same heap value in a single-threaded Rust program.

## Common wrong ideas to avoid
### Wrong idea 1
`Rc<T>` is for mutability.

No.
It is for shared ownership.

### Wrong idea 2
`Rc::clone()` copies the whole object deeply.

No.
It clones the pointer-like owner, not the inner data.

### Wrong idea 3
If something is `mut`, then multiple owners are fine.

No.
Mutation and ownership are separate ideas.

### Wrong idea 4
`Rc<T>` is mainly a threading feature.

No.
This project is specifically single-threaded.

## What success looks like
You know this project is successful if you can explain:

I used `Rc<AppConfig>` because several services needed to own the same application config at the same time, and `mut` would not solve that because the issue was multiple ownership, not mutation.
