use std::sync::Arc;

// One reason to use an immutable linked list is to share data across threads. After all, shared mutable state is the root of all evil, and one way to solve that is to kill the mutable part forever.

// Except our list isn't thread-safe at all. In order to be thread-safe, we need to fiddle with reference counts atomically. Otherwise, two threads could try to increment the reference count, and only one would happen. Then the list could get freed too soon!

// In order to get thread safety, we have to use Arc. Arc is completely identical to Rc except for the fact that reference counts are modified atomically. This has a bit of overhead if you don't need it, so Rust exposes both. All we need to do to make our list is replace every reference to Rc with std::sync::Arc. That's it. We're thread safe. Done!

// But this raises an interesting question: how do we know if a type is thread-safe or not? Can we accidentally mess up?

// No! You can't mess up thread-safety in Rust!

// The reason this is the case is because Rust models thread-safety in a first-class way with two traits: Send and Sync.

// A type is Send if it's safe to move to another thread. A type is Sync if it's safe to share between multiple threads. That is, if T is Sync, &T is Send. Safe in this case means it's impossible to cause data races, (not to be mistaken with the more general issue of race conditions).

// These are marker traits, which is a fancy way of saying they're traits that provide absolutely no interface. You either are Send, or you aren't. It's just a property other APIs can require. If you aren't appropriately Send, then it's statically impossible to be sent to a different thread! Sweet!

// Send and Sync are also automatically derived traits based on whether you are totally composed of Send and Sync types. It's similar to how you can only implement Copy if you're only made of Copy types, but then we just go ahead and implement it automatically if you are.

// Almost every type is Send and Sync. Most types are Send because they totally own their data. Most types are Sync because the only way to share data across threads is to put them behind a shared reference, which makes them immutable!

// However there are special types that violate these properties: those that have interior mutability. So far we've only really interacted with inherited mutability (AKA external mutability): the mutability of a value is inherited from the mutability of its container. That is, you can't just randomly mutate some field of a non-mutable value because you feel like it.

// Interior mutability types violate this: they let you mutate through a shared reference. There are two major classes of interior mutability: cells, which only work in a single-threaded context; and locks, which work in a multi-threaded context. For obvious reasons, cells are cheaper when you can use them. There's also atomics, which are primitives that act like a lock.

// So what does all of this have to do with Rc and Arc? Well, they both use interior mutability for their reference count. Worse, this reference count is shared between every instance! Rc just uses a cell, which means it's not thread safe. Arc uses an atomic, which means it is thread safe. Of course, you can't magically make a type thread safe by putting it in Arc. Arc can only derive thread-safety like any other type.

// I really really really don't want to get into the finer details of atomic memory models or non-derived Send implementations. Needless to say, as you get deeper into Rust's thread-safety story, stuff gets more complicated. As a high-level consumer, it all just works and you don't really need to think about it.

pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
      List { head: None }
  }

  pub fn append(&self, elem: T) -> List<T> {
    List { head: Some(Arc::new(Node {
      elem: elem,
      next: self.head.clone()
    }))}
  }

  pub fn tail(&self) -> List<T> {
    List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
  }

  pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem )
  }

  pub fn iter(&self) -> Iter<'_, T> {
    Iter { next: self.head.as_deref() }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut head = self.head.take();
    while let Some(node) = head {
      if let Ok(mut node) = Arc::try_unwrap(node) {
        head = node.next.take();
      } else {
        break;
      }
    }
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
      self.next.map(|node| {
          self.next = node.next.as_deref();
          &node.elem
      })
  }
}

// Note that we can't implement IntoIter or IterMut for this type. We only have shared access to elements.

#[cfg(test)]
mod test {
  use super::List;
  #[test]
  fn basics() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
  }

  #[test]
  fn iter() {
    let list = List::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }
}
