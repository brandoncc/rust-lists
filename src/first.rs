use std::mem;

type Value = i32;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    value: Value,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, value: Value) {
        let old_head = mem::replace(&mut self.head, Link::Empty);
        let new_node = Box::new(Node {
            value,
            next: old_head,
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<Value> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn values_can_be_pushed_and_popped() {
        let mut list = List::new();

        list.push(0x45);
        list.push(0x90);

        assert_eq!(Some(0x90), list.pop());
        assert_eq!(Some(0x45), list.pop());
    }

    #[test]
    fn values_can_be_popped_from_empty_lists() {
        let mut list = List::new();

        assert_eq!(None, list.pop());
    }
}
