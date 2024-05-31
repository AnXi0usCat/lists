use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_new() {
        // when
        let list = List::<i32>::new();
        // and_then
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_prepend() {
        // given
        let mut list = List::<i32>::new();
        // when
        list = list.prepend(1).prepend(2).prepend(3);
        // then
        assert_eq!(list.head(), Some(&3));
    }

    #[test]
    fn test_tail() {
        // given
        let mut list = List::<i32>::new();
        list = list.prepend(1).prepend(2).prepend(3);
        // when
        list = list.tail();
        assert_eq!(list.head(), Some(&2));
        list = list.tail();
        assert_eq!(list.head(), Some(&1));
        list = list.tail();
        assert_eq!(list.head(), None);
        // works with an empty list
        list = list.tail();
        assert_eq!(list.head(), None);
    }
}
