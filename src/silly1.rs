// Double Singly-Linked List
//
// we struggle with doubly-linked lists because they have tangled ownership semantics:
// no node strictly owns any other node. However we struggled with this because we brought
// preconceived notions of what a linked list is. Namely we assumed that all the links go in
// the same direction
//
// Instead, we can smash our list into two halves: one going left, and one going right


pub struct List<T> {
  left: Stack<T>,
  right: Stack<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { left: Stack::new(), right: Stack::new() }
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
     next: self.head.take(),
   });

   self.head = Some(new_node);
 }


}
