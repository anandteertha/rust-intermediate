## What is Shared Ownership  

In Rust, ownership is normally exclusive:
one value → one owner  

When ownership is transferred, the previous owner loses access  

---

## Problem Example (Without Rc)  

let doc = Document::new(...);  
let editor = TitleEditor::new(doc);  

Now:
- doc is moved  
- cannot use it again in main  
- cannot pass to another editor  

---

## Solution: Rc  

Rc allows:

let doc = Rc::new(Document::new(...));  

Now:
- multiple parts of program can own doc  
- cloning Rc does NOT duplicate data  

---

## Problem: Mutation  

With Rc<Document>:
- cannot mutate because only immutable access exists  

---

## Solution: RefCell  

RefCell allows:

- mutation through borrow_mut()  
- reads through borrow()  

This enables mutation even when Rc is shared  

---

## Combined Solution  

Rc<RefCell<T>>  

This gives:
- shared ownership  
- shared mutation  

---

## Real-Life Analogy  

Think of a shared Google Doc:

- many users (Rc owners)  
- document is one (shared state)  
- edits happen safely (RefCell control)  

---

## Important Limitation  

Rc<RefCell<T>> is:
- NOT thread-safe  

For multithreading, use:
- Arc<Mutex<T>>  

---

## Key Insight  

Rust separates:
- ownership  
- mutability  

Rc solves ownership  
RefCell solves mutability  

---

## Final Mental Model  

Document  
→ RefCell<Document>  
→ Rc<RefCell<Document>>  

Layered abstraction:
- data  
- mutability control  
- shared ownership  
