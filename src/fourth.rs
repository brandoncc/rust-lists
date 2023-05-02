use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> List<T> {
    pub fn push_front(&mut self, value: T) {
        let new_head = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.as_ref().borrow_mut().prev = Some(new_head.clone());
                new_head.as_ref().borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(new_head.clone());
            }
        }

        self.head = Some(new_head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.as_ref().borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.as_ref().borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_deref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.as_ref().borrow_mut().next = Some(new_tail.clone());
                new_tail.as_ref().borrow_mut().prev = Some(old_tail);
            }
            None => self.head = Some(new_tail.clone()),
        }

        self.tail = Some(new_tail);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.as_ref().borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.as_ref().borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_deref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.value))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_deref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.value))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_deref()
            .map(|head| RefMut::map(head.borrow_mut(), |node| &mut node.value))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_and_pop_front() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(Some(3), list.pop_front());
        assert_eq!(Some(2), list.pop_front());
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(None, list.pop_front());
    }

    #[test]
    fn pop_front_of_empty_list() {
        let mut list: List<i32> = List::new();

        assert_eq!(None, list.pop_front());
    }

    #[test]
    fn peek_front() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(Some(&3), list.peek_front().as_deref());
        assert_eq!(3, *list.peek_front().unwrap());
    }

    #[test]
    fn push_and_pop_back() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(Some(3), list.pop_back());
        assert_eq!(Some(2), list.pop_back());
        assert_eq!(Some(1), list.pop_back());
        assert_eq!(None, list.pop_back());
    }

    #[test]
    fn push_and_front_opposite_ends() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);

        assert_eq!(Some(1), list.pop_back());
        assert_eq!(Some(2), list.pop_back());
        assert_eq!(None, list.pop_back());
        assert_eq!(None, list.pop_front());

        list.push_back(3);
        list.push_back(4);

        assert_eq!(Some(3), list.pop_front());
        assert_eq!(Some(4), list.pop_front());
        assert_eq!(None, list.pop_front());
        assert_eq!(None, list.pop_back());
    }

    #[test]
    fn peek_back() {
        let mut list = List::new();

        list.push_front(1);

        assert_eq!(Some(&1), list.peek_back().as_deref());
    }

    #[test]
    fn peek_back_mut() {
        let mut list = List::new();

        list.push_front(1);
        let back = list.peek_back_mut();
        *back.unwrap() = 2;

        assert_eq!(Some(&2), list.peek_front().as_deref());
        assert_eq!(Some(&2), list.peek_back().as_deref());
    }

    #[test]
    fn peek_front_mut() {
        let mut list = List::new();

        list.push_front(1);
        let front = list.peek_front_mut();
        *front.unwrap() = 2;

        assert_eq!(Some(&2), list.peek_front().as_deref());
        assert_eq!(Some(&2), list.peek_back().as_deref());
    }

    #[test]
    fn into_iter_next() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        let mut iter = list.into_iter();

        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter_next_back() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        let mut iter = list.into_iter();

        assert_eq!(Some(1), iter.next_back());
        assert_eq!(Some(2), iter.next_back());
        assert_eq!(Some(3), iter.next_back());
        assert_eq!(Some(4), iter.next_back());
        assert_eq!(None, iter.next_back());
    }
}
