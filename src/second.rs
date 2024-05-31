pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
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
        let mut list = List::<i32>::new();
        // then
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_removal() {
        // given
        let mut list = List::<i32>::new();
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
        let mut list = List::<i32>::new();
        // when
        list.push(1);
        // then
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        // given
        let mut list = List::<i32>::new();
        // when
        list.push(1);
        // then
        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn test_peek_mut() {
        // given
        let mut list = List::<i32>::new();
        // when
        list.push(1);

        list.peek_mut().map(|value| *value = 42);
        // then
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn test_into_iter() {
        // given
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // when
        let mut iter = list.into_iter();
        // then
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        // given
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // when
        let mut iter = list.iter();
        // then
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn test_iter_mut() {
        // given
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // when
        let mut iter = list.iter_mut();
        // then
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
