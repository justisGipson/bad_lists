Shareable mutable containers.

Values of the `Cell<T>` and `RefCell<T>` types may be mutated through shared references (i.e. the common `&T` type), whereas most Rust types can only be mutated through unique (`&mut T`) references. We say that `Cell<T>` and `RefCell<T>` provide 'interior mutability', in contrast with typical Rust types that exhibit 'inherited mutability'.

Cell types come in two flavors: `Cell<T>` and `RefCell<T>`. `Cell<T>` provides `get` and `set` methods that change the interior value with a single method call. `Cell<T>` though is only compatible with types that implement `Copy`. For other types, one must use the `RefCell<T>` type, acquiring a write lock before mutating.

`RefCell<T>` uses Rust's lifetimes to implement 'dynamic borrowing', a process whereby one can claim temporary, exclusive, mutable access to the inner value. Borrows for `RefCell<T>`s are tracked 'at runtime', unlike Rust's native reference types which are entirely tracked statically, at compile time. Because `RefCell<T>` borrows are dynamic it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.

# [When to choose interior mutability](https://rust-unofficial.github.io/too-many-lists/fourth-building.html#when-to-choose-interior-mutability)

The more common inherited mutability, where one must have unique access to mutate a value, is one of the key language elements that enables Rust to reason strongly about pointer aliasing, statically preventing crash bugs. Because of that, inherited mutability is preferred, and interior mutability is something of a last resort. Since cell types enable mutation where it would otherwise be disallowed though, there are occasions when interior mutability might be appropriate, or even *must* be used, e.g.

- Introducing inherited mutability roots to shared types.
- Implementation details of logically-immutable methods.
- Mutating implementations of `Clone`.

## [Introducing inherited mutability roots to shared types](https://rust-unofficial.github.io/too-many-lists/fourth-building.html#introducing-inherited-mutability-roots-to-shared-types)

Shared smart pointer types, including `Rc<T>` and `Arc<T>`, provide containers that can be cloned and shared between multiple parties. Because the contained values may be multiply-aliased, they can only be borrowed as shared references, not mutable references. Without cells it would be impossible to mutate data inside of shared boxes at all!

It's very common then to put a `RefCell<T>` inside shared pointer types to reintroduce mutability:

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
```

Note that this example uses `Rc<T>` and not `Arc<T>`. `RefCell<T>`s are for single-threaded scenarios. Consider using `Mutex<T>` if you need shared mutability in a multi-threaded situation.
