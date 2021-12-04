use std::iter::FromIterator;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            val: element,
            next: self.head.take(),
        }));

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            self.size -= 1;
            let new_head = old_head.next;
            self.head = new_head;
            return Some(old_head.val);
        }

        None
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.val)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut curr = self.head.take();
        let mut parent = None;

        while let Some(mut curr_node) = curr {
            let next = curr_node.next.take();
            curr_node.next = parent;
            parent = Some(curr_node);
            curr = next;
        }

        self.head = parent;
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = SimpleLinkedList::new();
        iter.into_iter().for_each(|item| result.push(item));
        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        while let Some(item) = self.pop() {
            result.push(item);
        }
        result.reverse();
        result
    }
}
