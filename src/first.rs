use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_link {
          current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_empty_list() {
        // given
        let mut list = List::new();
        // then
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_removal() {
        // given
        let mut list = List::new();
        // when
        list.push(1);
        list.push(2);
        list.push(3);
        // then
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn test_exhaustion() {
        // given
        let mut list = List::new();
        // when
        list.push(1);
        // then
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
