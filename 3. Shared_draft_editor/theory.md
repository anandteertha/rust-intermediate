# Theory - Project 003

## What This Project Is Really About

This project teaches one of Rust's most important design ideas:

**ownership and mutability are separate problems**

Project 2 solved shared ownership with `Rc<T>`.  
Project 3 asks the next systems question:

- what if several parts of one program need to share the same value
- and some of them also need to mutate it?

That is why this project uses `Rc<RefCell<T>>`.

## The Systems Problem

The project models a shared document workflow.

Different actors interact with the same document:

- a title editor changes the title
- a body editor appends content
- a reviewer reads the final result

Architecturally, there is only one document.  
But multiple components need access to it, and some need write access.

This is a common pattern in stateful applications:

- editors
- workflows
- UI models
- in-memory state machines

## Why `Rc<T>` Alone Is Not Enough

`Rc<T>` gives shared ownership, but only shared immutable access.

That means several components can point to the same value, but they still cannot mutate it through ordinary borrowing rules.

This is a very important Rust lesson:

- shared ownership does not automatically imply shared mutation

If a document is shared across several owners, Rust becomes conservative about mutable access because aliasing plus mutation is dangerous.

## What `RefCell<T>` Adds

`RefCell<T>` introduces interior mutability.

That means:

- the outer owner may appear immutable
- the inner value can still be mutably borrowed at runtime

Instead of enforcing all borrow rules only at compile time, `RefCell<T>` enforces them at runtime.

So this project chooses:

- compile-time shared ownership with `Rc<T>`
- runtime borrow checking with `RefCell<T>`

## Why This Is A Useful Systems Tradeoff

This pattern is useful when:

- the program is single-threaded
- there is one shared piece of state
- multiple components need coordinated access
- strict compile-time borrowing would make the architecture awkward

The point is not to bypass safety casually.  
The point is to model a shared mutable workflow honestly inside a constrained, single-threaded environment.

## The Layered Abstraction

The real mental model is:

- `Document` is the data
- `RefCell<Document>` controls mutable access
- `Rc<RefCell<Document>>` lets several owners share that access point

Each layer solves a different problem:

- the struct models the domain
- `RefCell` models mutable access control
- `Rc` models shared ownership

That layering is one of the most important intermediate Rust ideas.

## Why Runtime Borrow Checking Matters

`RefCell<T>` is safe, but its safety works differently from normal references.

Normal references:

- are checked fully at compile time

`RefCell<T>`:

- checks borrow rules at runtime

That means the program can panic if it tries to:

- borrow mutably while an immutable borrow is still active
- borrow twice mutably at the same time

So this project also teaches a deeper engineering lesson:

**some flexibility is possible, but it moves certain guarantees from compile time to runtime**

## Why This Project Is Still Single-Threaded

`Rc<RefCell<T>>` is not thread-safe.

That is not a weakness of the project.  
It is part of the learning progression.

The project stays focused on:

- shared ownership
- shared mutation
- one thread of execution

If you cross into multithreading, the design changes and you need thread-safe primitives such as:

- `Arc<T>`
- `Mutex<T>`

That is exactly what the next stage of the repository explores.

## What The Editor Roles Teach

The project does something useful architecturally:

- mutation is performed through specialized components
- reading is performed through a reviewer

That models a capability-based design:

- some actors edit
- some actors observe

Even though the code is small, this is a real systems idea.  
Large systems often become easier to reason about when responsibilities are separated by role.

## Why Versioning Matters

The document increments a version whenever it changes.

That hints at another useful systems concept:

- state transitions should be observable

Versioning helps show:

- when state changed
- how many writes happened
- that mutations are not invisible side effects

This is a very small example of state tracking.

## Limits Of This Design

This pattern is useful, but it has clear boundaries:

- single-threaded only
- runtime borrow violations can panic
- too much shared mutable state can still make code hard to reason about

So `Rc<RefCell<T>>` is a powerful tool, but it is best used deliberately.

## Final Mental Model

The best way to think about project 3 is:

- there is one shared document
- multiple components need to own access to it
- some components must mutate it
- `Rc<RefCell<Document>>` provides shared ownership plus controlled interior mutation in a single-threaded program

That is the core systems lesson behind the code.
