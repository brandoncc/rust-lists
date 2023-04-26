use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn prepend(&self, value: T) -> List<T> {
        let new_node = Some(Rc::new(Node {
            value,
            next: self.head.clone(),
        }));

        let new_list = List { head: new_node };

        new_list
    }

    fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|head| head.next.clone()),
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.as_ref().clone().value)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn prepend() {
        let mut list = List::new();

        list = list.prepend(1);
        assert_eq!(Some(&1), list.head());

        list = list.prepend(2);
        assert_eq!(Some(&2), list.head());

        list = list.prepend(3);
        assert_eq!(Some(&3), list.head());
    }

    #[test]
    fn tail() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let tail = list.tail();

        assert_eq!(Some(&2), tail.head());
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
    }
}
