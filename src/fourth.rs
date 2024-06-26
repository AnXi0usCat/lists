use std::{cell::RefCell, rc::Rc};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_empty_list() {
        // given
        let mut list = List::<i32>::new();
        // then
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_populate() {
        // given
        let mut list = List::<i32>::new();
        // when
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        // then
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_exhaustion() {
        // given
        let mut list = List::<i32>::new();
        // when
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        // then
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
