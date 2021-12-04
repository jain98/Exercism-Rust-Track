use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type OptNode<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub prev: OptNode<T>,
    pub next: OptNode<T>,
}

pub struct LinkedList<T> {
    head: OptNode<T>,
    tail: OptNode<T>,
    n: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    curr: OptNode<T>,
}

pub struct Iter<'a, T> {
    head: &'a OptNode<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> NonNull<Self> {
        let result = Node {
            value,
            prev: None,
            next: None,
        };

        unsafe {
            // Deliberately leak memory here. Later on in take,
            // we will make sure to `drop` the object
            // pointed at by the returned pointer.
            NonNull::new_unchecked(Box::into_raw(Box::new(result)))
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            n: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor::new(self, self.head)
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor::new(self, self.tail)
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: &self.head,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<'a, T> Cursor<'a, T> {
    fn new(list: &'a mut LinkedList<T>, curr: OptNode<T>) -> Self {
        Cursor {
            list,
            curr,
        }
    }

    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.curr.as_mut().map(|node| unsafe {
            // TODO: Need this for the type to be sync/send???
            //  If we want to send this across threads we cannot enforce Rust's aliasing rules -
            //  https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.as_mut
            //&mut node.as_mut().value
            &mut (*node.as_ptr()).value
        })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if let Some(node) = self.curr {
            unsafe {
                (*node.as_ptr()).next.map(|node| {
                    // NonNull<T> is `Copy` when T is ?Sized (may or may not be sized).
                    // NonNull<T> is just a raw pointer to T. And since raw pointers don't
                    // have to uphold Rust's safety or liveness guarantees, they can be
                    // copied without the compiler complaining.
                    self.curr = Some(node);
                    &mut (*node.as_ptr()).value
                })
            }
        } else {
            None
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if let Some(node) = self.curr {
            unsafe {
                (*node.as_ptr()).prev.map(|node| {
                    // NonNull<T> is `Copy` when T is ?Sized (may or may not be sized).
                    // NonNull<T> is just a raw pointer to T. And since raw pointers don't
                    // have to uphold Rust's safety or liveness guarantees, they can be
                    // copied without the compiler complaining.
                    self.curr = Some(node);
                    &mut (*node.as_ptr()).value
                })
            }
        } else {
            None
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if let Some(curr) = self.curr { //.map(|curr| unsafe {
            unsafe {
                let curr_raw_ptr = curr.as_ptr();
                let curr_prev_ptr = (*curr_raw_ptr).prev;
                let curr_next_ptr = (*curr_raw_ptr).next;

                match (curr_prev_ptr, curr_next_ptr) {
                    (Some(prev), Some(next)) => { // Removing a non-terminal node
                        let (prev_raw_ptr, next_raw_ptr) = (prev.as_ptr(), next.as_ptr());
                        (*prev_raw_ptr).next.replace(next);
                        (*next_raw_ptr).prev.replace(prev);
                        self.curr.replace(next);
                    },
                    (None, Some(next)) => { // Removing head
                        let next_raw_ptr = next.as_ptr();
                        (*next_raw_ptr).prev.take();
                        self.curr.replace(next);
                        self.list.head.replace(next);
                    },
                    (Some(prev), None) => { // Removing tail
                        let prev_raw_ptr = prev.as_ptr();
                        (*prev_raw_ptr).next.take();
                        self.curr.replace(prev);
                        self.list.tail.replace(prev);
                    },
                    (None, None) => { // Removing the only node in the list
                        self.list.head.take();
                        self.list.tail.take();
                        self.curr.take();
                    },
                }

                self.list.n -= 1;

                // This should move `value` out of the node that cursor was pointing to at the
                // beginning of the method call and drop the node itself.
                // TODO: Why does this fail the memory leak tests?
                // Some(std::ptr::read(curr_raw_ptr).value);
                Some(Box::from_raw(curr_raw_ptr).value)
            }
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            match self.curr {
                Some(curr) => {
                    let new_node_ptr = Node::new(element);
                    let new_node_raw_ptr = new_node_ptr.as_ptr();
                    let curr_raw_ptr = curr.as_ptr();
                    (*new_node_raw_ptr).prev.replace(curr);
                    let curr_next_ptr = (*curr_raw_ptr).next.replace(new_node_ptr);

                    match curr_next_ptr {
                        Some(curr_next_ptr) => {
                            let curr_next_raw_ptr = curr_next_ptr.as_ptr();
                            (*curr_next_raw_ptr).prev.replace(new_node_ptr);
                            (*new_node_raw_ptr).next.replace(curr_next_ptr);
                        },
                        None => {
                            self.list.tail.replace(new_node_ptr);
                        },
                    }
                },
                None => {
                    let new_node_ptr = Node::new(element);
                    self.list.head.replace(new_node_ptr);
                    self.list.tail.replace(new_node_ptr);
                },
            }
        }
        self.list.n += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            match self.curr {
                Some(curr) => {
                    let new_node_ptr = Node::new(element);
                    let new_node_raw_ptr = new_node_ptr.as_ptr();
                    let curr_raw_ptr = curr.as_ptr();
                    (*new_node_raw_ptr).next.replace(curr);
                    let curr_prev_ptr = (*curr_raw_ptr).prev.replace(new_node_ptr);

                    match curr_prev_ptr {
                        Some(curr_prev_ptr) => {
                            let curr_prev_raw_ptr = curr_prev_ptr.as_ptr();
                            (*curr_prev_raw_ptr).next.replace(new_node_ptr);
                            (*new_node_raw_ptr).prev.replace(curr_prev_ptr);
                        },
                        None => {
                            self.list.head.replace(new_node_ptr);
                        }
                    }
                },
                None => {
                    let new_node_ptr = Node::new(element);
                    self.list.head.replace(new_node_ptr);
                    self.list.tail.replace(new_node_ptr);
                },
            }
        }
        self.list.n += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.head {
            Some(node) => unsafe {
                let node_raw_ptr = node.as_ptr();
                let result = &(*node_raw_ptr).value;
                self.head = &(*node_raw_ptr).next;
                Some(result)
            }
            None => None,
        }
    }
}
