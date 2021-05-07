// So what's a singly-linked queue like? Well, when we had a singly-linked stack we pushed onto one end of the list,
// and then popped off the same end. The only difference between a stack and a queue is that a queue pops off the other
// end.
//
// So from our stack implementation we have:
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// stack push X:
// [Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)
//
// stack pop:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// To make a queue, we just need to decide which operation to move to the end of the list: push, or pop? Since our list
// is singly-linked, we can actually move either operation to the end with the same amount of effort.
//
// To move push to the end, we just walk all the way to the None and set it to Some with the new element.
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// flipped push X:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//
// To move pop to the end, we just walk all the way to the node before the None, and take it:
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//
// flipped pop:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// One key observation is that we're wasting a ton of work doing the same thing over and over. Can we memoize this
// work? Why, yes! We can store a pointer to the end of the list, and just jump straight to there!

use std::ptr;

pub struct List<T> {
  head: Link<T>,
  tail: *mut Node<T> // DANGER ZONE
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<'a, T> List<T> {
  pub fn new() -> Self {
    List { head: None, tail: ptr::null_mut()}
  }

  pub fn push(&'a mut self, elem: T) {
    let mut new_tail = Box::new(Node {
      elem: elem,
      // when you push onto the tail, your next is ALWAYS None
      next: None,
    });

    // swap old_tail to point to new_tail
    let raw_tail: *mut _ = &mut *new_tail;

    if !self.tail.is_null() {
      // if the old tail existed, update to point to new_tail
      unsafe {
        (*self.tail).next = Some(new_tail);
      }
    } else {
      // otherwise update head to point to it
      self.head = Some(new_tail);
    }
    self.tail = raw_tail;
  }

  pub fn pop(& mut self) -> Option<T> {
    // grab the lists current head
    self.head.take().map(|head| {
      let head = *head;
      self.head = head.next;

      // if we're out of 'head', make sure to set the tail to None
      if self.head.is_none() {
        self.tail = ptr::null_mut();
      }
      head.elem
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
    assert_eq!(list.pop(), None);

    // populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // check normal removal
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));

    // push more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));

    // check exhaustion
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), None);

    // check the exhaustion case fixed the pointer
    list.push(6);
    list.push(7);

    // check normal removal
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), None);
  }
}
