BAD LINKED LISTS

![unsafe.svg](unsafe.svg)

Like, real bad... this is a terrible idea, they're terrible data structures.

But here's some GREAT use cases for this bad idea:
- You want to do *a lot* of splitting or merging of big lists. *A LOT*
- You're doing some awesome lock-free concurrent thing
- You're writing a kernel/embedded thing and want to use an intrusive list
- You're using a pure functional language and the limited semantics and absence of mutation makes linked lists easier to work with


99% of the time, use Vec(array stack).
99% of the other 1% of the time, use VecDeque(array deque)

Due to less frequent allocation, lower memory overhead, true random access, and cache locality - they are superior structure for most workloads.
