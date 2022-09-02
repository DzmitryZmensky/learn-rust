use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>
}

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList::<T> {
        LinkedList::<T> { head: None }
    }

    pub fn add_front(&mut self, value: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node::new(value, old_head)));
    }

    pub fn add_tail(&mut self, value: T) {
        let new_link = Some(Box::new(Node::new(value, None)));
        if self.head.is_none() {
            self.head = new_link;
            return;
        }

        let mut cur = &mut self.head;
        while cur.as_ref().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        };
        cur.as_mut().unwrap().next = new_link;
    }

    pub fn delete_front(&mut self) {
        match self.head.take() {
            Some(n) => {
                self.head = n.next;
            }
            None => {}
        }
    }

    pub fn iter(&self) -> LinkedListIter::<T> {
        LinkedListIter::<T>{ cur: &self.head }
    }
}

pub struct LinkedListIter<'a, T> {
    cur: &'a Link<T>
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Some(n) => {
                let result = Some(&n.value);
                self.cur = &n.next;
                result
            }
            None => None
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cur = &self.head;
        while let Some(n) = cur {
            write!(f, "{}->", n.value)?;
            cur = &n.next;
        }
        Ok(())
    }
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Node<T> {
        Node::<T> { value, next }
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    use core::fmt::Debug;

    fn assert_equal<T>(expected: &[T], actual: &LinkedList<T>) where T: Eq + Debug {
        let mut actual_iter = actual.iter();
        for expected_item in expected {
            match actual_iter.next(){
                Some(actual_item) => {
                    assert_eq!(expected_item, actual_item);
                }
                None => {
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn add_front() {
        let mut list = LinkedList::new();
        assert_equal(&[], &list);
        list.add_front(0);
        assert_equal(&[0], &list);
        list.add_front(1);
        assert_equal(&[1,0], &list);
        assert_equal(&[1,0], &list);
    }

    #[test]
    fn add_tail() {
        let mut list = LinkedList::new();
        
        list.add_tail(0);
        assert_equal(&[0], &list);
        list.add_tail(1);
        assert_equal(&[0,1], &list);
    }

    #[test]
    fn delete_front() {
        let mut list = LinkedList::new();
        list.delete_front();
        list.add_front(0);
        list.add_front(1);
        assert_equal(&[1,0], &list);
        list.delete_front();
        assert_equal(&[0], &list);
        list.delete_front();
        assert_equal(&[], &list);
    }
}
