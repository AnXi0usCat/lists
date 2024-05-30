pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
          current_link = boxed_node.next.take();
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
