use std::iter::FromIterator;
use std::mem;

#[derive(Debug)]
pub struct Node<T> {
    data: T, 
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, size: 0 } 
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // https://rust-unofficial.github.io/too-many-lists/first-push.html
    pub fn push(&mut self, _element: T) {
        let mut new_node = Box::new(Node::new(_element));
        // Move old self.head to new_node.next
        // Replace self.head with None
        new_node.next = mem::replace(&mut self.head, None);
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        // Move old self.head to top, replace self.head with None
        let top = mem::replace(&mut self.head, None).unwrap();
        self.head = top.next; // Change self.head to old self.head.next
        self.size -= 1;
        Some(top.data)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() { return None; }
        Some(&self.head.as_ref().unwrap().data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let v:Vec<T> = self.into();
        let mut s:SimpleLinkedList<T> = SimpleLinkedList::new();
        for e in v.into_iter().rev() { s.push(e); }
        s
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ret: SimpleLinkedList<T> = SimpleLinkedList::new();
        for i in _iter {
            ret.push(i);
        }
        ret
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();
        while !_linked_list.is_empty() {
            ret.push(_linked_list.pop().unwrap());
        }
        ret.into_iter().rev().collect()
    }
}

