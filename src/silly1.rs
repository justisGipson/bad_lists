// Double Singly-Linked List
//
// we struggle with doubly-linked lists because they have tangled ownership semantics:
// no node strictly owns any other node. However we struggled with this because we brought
// preconceived notions of what a linked list is. Namely we assumed that all the links go in
// the same direction
//
// Instead, we can smash our list into two halves: one going left, and one going right
//
// This is an extreme example of a finger data structure, where we maintain some kind of finger into the structure, and
// as a consequence can support operations on locations in time proportional to the distance from the finger.
//
// We can make very fast changes to the list around our finger, but if we want to make changes far away from our finger
// we have to walk all the way over there. We can permanently walk over there by shifting the elements from one stack
// to the other, or we could just walk along the links with an &mut temporarily to do the changes. However the &mut can
// never go back up the list, while our finger can!


pub struct List<T> {
  left: Stack<T>,
  right: Stack<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
      List { left: Stack::new(), right: Stack::new() }
  }

  pub fn push_left(&mut self, elem: T) {
    self.left.push(elem)
  }

  pub fn push_right(&mut self, elem: T) {
    self.right.push(elem)
  }

  pub fn pop_left(&mut self) -> Option<T> {
    self.left.pop()
  }

  pub fn pop_right(&mut self) -> Option<T> {
    self.right.pop()
  }

  pub fn peek_left(&self) -> Option<&T> {
    self.left.peek()
  }

  pub fn peek_right(&self) -> Option<&T> {
    self.right.peek()
  }

  pub fn peek_left_mut(&mut self) -> Option<&mut T> {
    self.left.peek_mut()
  }

  pub fn peek_right_mut(&mut self) -> Option<&mut T> {
    self.right.peek_mut()
  }

  pub fn go_left(&mut self) -> bool {
    self.left.pop_node().map(|node| {
      self.right.push_node(node);
    }).is_some()
  }

  pub fn go_right(&mut self) -> bool {
    self.right.pop_node().map(|node| {
      self.left.push_node(node);
    }).is_some()
  }
}

pub struct Stack<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Stack { head: None }
  }

 pub fn push(&mut self, elem: T) {
   let new_node = Box::new(Node {
     elem: elem,
     next: None,
   });

   self.push_node(new_node);
 }

 fn push_node(&mut self, mut node: Box<Node<T>>) {
   node.next = self.head.take();
   self.head = Some(node);
 }

 pub fn pop(&mut self) -> Option<T> {
   self.pop_node().map(|node| {
     node.elem
   })
 }

 fn pop_node(&mut self) -> Option<Box<Node<T>>> {
  self.head.take().map(|mut node| {
    self.head = node.next.take();
    node
  })
 }

 pub fn peek(&self) -> Option<&T> {
  self.head.as_ref().map(|node| {
    &node.elem
  })
 }

 pub fn peek_mut(&mut self) -> Option<&mut T> {
   self.head.as_mut().map(|node| {
     &mut node.elem
   })
 }
}

impl<T> Drop for Stack<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn walk_aboot() {
    let mut list = List::new();               // [_]

    list.push_left(0);                        // [0,_]
    list.push_right(1);                       // [0,_1]
    assert_eq!(list.peek_left(), Some(&0));
    assert_eq!(list.peek_right(), Some(&1));

    list.push_left(2);                        // [0, 2, _, 1]
    list.push_left(3);                        // [0, 2, 3, _, 1]
    list.push_right(4);                       // [0, 2, 3, _, 4, 1]

    while list.go_left() {}                   // [_, 0, 2, 3, 4, 1]

    assert_eq!(list.pop_left(), None);
    assert_eq!(list.pop_right(), Some(0));    // [_, 2, 3, 4, 1]
    assert_eq!(list.pop_right(), Some(2));    // [_, 3, 4, 1]

    list.push_left(5);                        // [5, _, 3, 4, 1]

    assert_eq!(list.pop_right(), Some(3));    // [5, _, 4, 1]
    assert_eq!(list.pop_left(), Some(5));     // [_, 4, 1]
    assert_eq!(list.pop_right(), Some(4));    // [_, 1]
    assert_eq!(list.pop_right(), Some(1));    // [_]

    assert_eq!(list.pop_right(), None);
    assert_eq!(list.pop_left(), None);
  }
}
