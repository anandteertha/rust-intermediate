# Theory - Project 002

## What This Project Is Really About

This project is about shared ownership in a single-threaded system.

The important question is not:

- "How do I mutate config?"

The real question is:

- "How can multiple parts of a program all keep owning access to the same configuration?"

That is the problem `Rc<T>` solves.

## The Systems Problem

Many programs have one application configuration that needs to be visible across multiple components.

Typical examples include:

- environment name
- database URL
- retry policy
- feature flags
- service endpoints

Architecturally, that config is one shared piece of system context.

But Rust ownership is exclusive by default:

- one value
- one owner

That default is safe, but it does not directly model shared read access across many components.

## Why This Is A Systems Design Question

At the architecture level, a config object is usually:

- global in meaning
- shared in usage
- mostly read-only

If each service deep-copies the config, the design becomes less honest:

- duplication increases
- updates become harder to reason about
- the code suggests many configs even though conceptually there is one

This project models the system more accurately:

- one config value
- several owners of handles to that value

## What `Rc<T>` Represents

`Rc<T>` is not about mutation.

It is about:

- shared ownership
- reference counting
- one underlying allocation with several owners

When a service receives `Rc<AppConfig>`, it is not getting a cloned config object.  
It is getting another owner of the same logical config.

That matters because the code now matches the system story.

## Why This Project Stays Single-Threaded

That choice is deliberate.

Before you mix in concurrency, you want to understand the ownership problem clearly:

- several components need access
- nobody needs independent copies
- nobody needs mutation yet

`Rc<T>` is perfect for that stage because it teaches shared ownership without also introducing thread-safety or locking.

## Why `mut` Is Not The Answer

This project is also a good correction to a very common misunderstanding.

`mut` does not solve:

- multiple ownership
- move semantics
- shared access across components

`mut` only changes whether a binding can mutate through its current owner.

So if three services all need to own config access, the issue is ownership structure, not mutation capability.

## Why Read-Only Shared Context Is So Common

In many real systems, shared configuration is best treated as immutable after startup.

That gives several benefits:

- components behave consistently
- reasoning becomes easier
- accidental divergence is avoided

This project reflects that common pattern:

- build config once
- share it widely
- read from it wherever needed

## How This Project Models Service Architecture

The project has three services:

- auth
- payments
- API gateway

Each depends on application configuration, but none owns the configuration itself.

That is a useful systems lesson:

**dependencies should be provided, not recreated**

This makes the program feel more like a service-oriented architecture, even though it is very small.

## Why `Rc::clone()` Is Conceptually Important

`Rc::clone()` is a great example of Rust making an architectural idea explicit.

It does not mean:

- duplicate the inner config

It means:

- create another owner of the same shared context

That is exactly what many real systems need when wiring dependencies into components.

## Limits Of This Design

This design is intentionally limited in a good way:

- single-threaded only
- read-focused
- no shared mutation

Those limits are useful because they keep the lesson focused on one abstraction:

**shared ownership without shared mutation**

## What This Project Leads To Next

Once you understand `Rc<T>`, the next question becomes:

- what if multiple owners also need mutation?

That leads naturally to:

- `RefCell<T>` for single-threaded interior mutability
- `Rc<RefCell<T>>` for shared mutable single-threaded state
- `Arc<Mutex<T>>` for shared mutable multithreaded state

So project 2 is an important bridge, not just an isolated exercise.

## Final Mental Model

The best way to think about project 2 is:

- one application context exists
- several components need to keep owning access to it
- the context is shared, not copied
- `Rc<T>` models that shared ownership honestly in single-threaded Rust

That is the core systems lesson behind the code.
