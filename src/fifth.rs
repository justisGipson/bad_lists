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

pub struct List<T> {
  head: Link<T>,
  tail: Option<&'a mut Node<T>>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None, tail: None}
  }

  pub fn push(&mut self, elem: T) {
    let new_tail = Box::new(Node {
      elem: elem,
      // when you push onto the tail, your next is ALWAYS None
      next: None,
    });

    // swap old_tail to point to new_tail
    let new_tail = match self.tail.take() {
      Some(mut old_tail) => {
        // if old_tail existed, update to point to new_tail
        old_tail.next = Some(new_tail);
        old_tail.next.as_deref_mut()
      }
      None => {
        self.head = Some(new_tail);
        self.head.as_deref_mut()
      }
    };
    self.tail = new_tail;
  }
}
