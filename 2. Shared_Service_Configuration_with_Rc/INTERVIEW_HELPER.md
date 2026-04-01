# Rust Intermediate Project 002 - Interview Helper

## One-line project explanation
I built a small Rust project where multiple services shared ownership of one application configuration using `Rc<T>`.

## What problem this project solves
Rust normally prefers one owner per value.

That is great for safety, but sometimes several parts of the same program all need to keep using the same value.

In this project, multiple services needed access to the same config, so shared ownership was the real problem to solve.

## Why `mut` was not the answer
`mut` only controls whether a binding can be changed.

It does not let multiple structs own the same value.

So the issue was not mutation.
The issue was ownership.

## What `Rc<T>` does
`Rc<T>` allows multiple owners of the same heap-allocated value in a single-threaded Rust program.

When you clone an `Rc<T>`, you are not deep-copying the inner data.
You are creating another owner of the same underlying value.

## Best explanation of `Rc::clone()`
A very strong explanation is:

`Rc::clone()` creates another shared owner of the same value. It does not duplicate the full inner object.

## Why this project is single-threaded
This project is about shared ownership only.

It intentionally avoids adding other concepts so the core idea stays clean.

## What to say if asked why Rc was needed
I needed `Rc<T>` because more than one service had to own the same configuration at the same time, and Rust's normal ownership model does not allow multiple owners of the same value by default.

## What to say if asked why not just copy the config
You could copy small configs, but that hides the real ownership lesson. The point of the project was to learn how Rust models shared ownership explicitly rather than duplicating data blindly.

## What to say if asked what you learned
I learned that shared ownership and mutation are different ideas in Rust. `mut` is about changing a value through a binding, while `Rc<T>` is about allowing multiple owners of the same value.

## Good short answer
This project taught me how Rust handles shared ownership in a single-threaded design. I used `Rc<T>` so multiple services could own the same application config without deep-copying it.

## Resume-style line
Built a small Rust application where multiple services shared one application configuration using `Rc<T>`, strengthening my understanding of shared ownership, heap allocation, and ownership modeling in single-threaded system design.
