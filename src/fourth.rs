// Disclaimer: this is basically a demonstration that this is a very bad idea.

// The key to our design is the RefCell type. The heart of RefCell is a pair of methods:
//
//
// fn borrow(&self) -> Ref<'_, T>;
// fn borrow_mut(&self) -> RefMut<'_, T>;
//
// The rules for borrow and borrow_mut are exactly those of & and &mut: you can call borrow as many times as you want, but borrow_mut requires exclusivity.
//
// Rather than enforcing this statically, RefCell enforces them at runtime. If you break the rules, RefCell will just panic and crash the program. Why does it return these Ref and RefMut things? Well, they basically behave like Rcs but for borrowing. They also keep the RefCell borrowed until they go out of scope. We'll get to that later.
//
// Now with Rc and RefCell we can become... an incredibly verbose pervasively mutable garbage collected language that can't collect cycles! Y-yaaaaay...
//
// Alright, we want to be doubly-linked. This means each node has a pointer to the previous and next node. Also, the list itself has a pointer to the first and last node. This gives us fast insertion and removal on both ends of the list.

use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
  head: Link<T>,
  tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
  prev: Link<T>,
}

impl<T> Node<T> {
  fn new(elem: T) -> Rc<RefCell<Self>>{
    Rc::new(RefCell::new(Node {
      elem: elem,
      next: None,
      prev: None,
    }))
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head:None, tail: None}
  }

  pub fn push_front(&mut self, elem: T) {
    let new_head = Node::new(elem);

    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().prev = Some(new_head.clone());
        new_head.borrow_mut().next = Some(old_head);
        self.head = Some(new_head);
      }
      None => {
        self.tail = Some(new_head.clone());
        self.head = Some(new_head);
      }
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
      match old_head.borrow_mut().next.take() {
        Some(new_head) => {
          new_head.borrow_mut().prev.take();
          self.head = Some(new_head);
        }
        None => {
          self.tail.take();
        }
      }
      Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
    })
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();
    // empty list behavior check
    assert_eq!(list.pop_front(), None);
    // populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    // check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    // push more to list, check nothing is corrupted
    list.push_front(4);
    list.push_front(5);
    // check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));
    // check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
  }
}
