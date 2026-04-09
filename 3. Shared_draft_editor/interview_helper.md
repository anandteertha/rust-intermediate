## When to use Rc<RefCell<T>>  

Use Rc<RefCell<T>> when:
- you need multiple owners of the same data  
- you need to mutate that data  
- your program is single-threaded  

---

## When NOT to use it  

Do NOT use when:
- working with multiple threads → use Arc<Mutex<T>>  
- simple ownership works → avoid unnecessary complexity  

---

## Core Idea  

Rc gives shared ownership  
RefCell gives interior mutability  

Together:
multiple owners can safely mutate shared state  

---

## Borrow Rules (Runtime)  

RefCell enforces:
- many immutable borrows allowed  
- OR one mutable borrow  
- not both  

Violation causes panic at runtime  

---

## Common Mistake  

Trying to pass:

document.borrow()

Instead of:

Rc<RefCell<Document>>

Borrow is temporary access  
Rc is ownership handle  

---

## Key Difference  

Compile-time borrowing:
- strict  
- safe  
- inflexible  

Runtime borrowing (RefCell):
- flexible  
- checked at runtime  
- can panic  

---

## Interview Explanation  

“I use Rc<RefCell<T>> when I need multiple owners of a value in a single-threaded program and I also need mutation. Rc provides shared ownership, and RefCell allows mutation through runtime borrow checking.”
