Project 003: Shared Draft Editor (Rc<RefCell<T>>)

## Overview  
This project demonstrates shared ownership and interior mutability in Rust using Rc and RefCell.

We build a simple document editing system where multiple components:
- TitleEditor  
- BodyEditor  
- Reviewer  

operate on the same underlying Document.

The key idea is:  
multiple owners + shared mutation in a single-threaded context.

---

## Problem  
In Rust, you cannot:
- have multiple owners of data (without Rc), and  
- mutate shared data safely across owners (without interior mutability)

This makes it difficult to design systems where multiple components need to read and modify the same state.

---

## Solution  
We use:

- Rc<T> for shared ownership  
- RefCell<T> for interior mutability (runtime borrow checking)

Combined:

Rc<RefCell<Document>>

This allows:
- multiple parts of the program to own the same document  
- safe mutation using borrow_mut()  
- safe reads using borrow()

---

## Project Structure  

- document.rs → core data model and logic  
- editor.rs → components operating on shared state  
- main.rs → orchestration  

---

## Execution Flow  

1. Create a Document  
2. Wrap it in Rc<RefCell<_>>  
3. Clone Rc into multiple editors  
4. Editors mutate the same document  
5. Reviewer reads and prints final state  

---

## Key Concepts  

### Rc (Reference Counting)  
- Enables multiple owners of the same data  
- Cloning Rc does NOT clone the data  
- Only increments a reference counter  

### RefCell (Interior Mutability)  
- Allows mutation even when the outer reference is immutable  
- Enforces borrow rules at runtime:  
  - many immutable borrows OR  
  - one mutable borrow  

### Rc<RefCell<T>>  
- Multiple owners  
- Shared mutable state  
- Single-threaded only  

---

## Why not just use normal references?  

Rust enforces:
- one mutable reference OR many immutable references at compile time  

This becomes restrictive when:
- multiple components need to mutate shared state  

RefCell shifts this checking to runtime, enabling more flexible designs.

---

## Example Use Cases  

### Shared Document Editing  
Multiple editors working on the same document instance  

### UI State Management  
Different UI components reading and updating shared state  

### Tree / Graph Structures  
Nodes sharing references to children or parents  

### Configuration Object  
Multiple services reading/updating shared configuration  

---

## Without Rc (what goes wrong)  

If you write:

let doc = Document::new(...);  
let editor = TitleEditor::new(doc);

Then:
- ownership is moved into TitleEditor  
- main loses access  
- cannot pass to other components  

---

## With Rc  

let doc = Rc::new(...);

Now:
- Rc::clone(&doc) allows multiple owners  
- all point to the same underlying data  

---

## Without RefCell (what goes wrong)  

With Rc<Document>:
- shared ownership exists  
- but mutation is not allowed  

---

## With RefCell  

Rc<RefCell<Document>> allows:

- borrow() for reading  
- borrow_mut() for mutation  

Now shared mutation is possible.

---

## Final Takeaway  

Rc solves shared ownership  
RefCell solves shared mutation  

Together:  
Rc<RefCell<T>> enables shared mutable state in single-threaded Rust  
