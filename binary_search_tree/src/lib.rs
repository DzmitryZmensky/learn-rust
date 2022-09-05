use core::panic;
use std::{cell::RefCell, rc::Rc};

type MaybeBoxedNode<ValueType> = Option<Rc<RefCell<BNode<ValueType>>>>;

pub struct BST<ValueType: PartialOrd> {
    root: MaybeBoxedNode<ValueType>,
}

struct BNode<ValueType: PartialOrd> {
    pub value: ValueType,
    pub left: MaybeBoxedNode<ValueType>,
    pub right: MaybeBoxedNode<ValueType>,
}

impl<ValueType: PartialOrd> BST<ValueType> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn contains(&self, value: ValueType) -> bool {
        let mut cur = self.root.clone();
        while let Some(boxed) = cur {
            let node_value = &boxed.borrow().value;            
            if *node_value > value {
                cur = boxed.borrow().left.clone();
            } else if *node_value < value{
                cur = boxed.borrow().right.clone();
            } else {
                return true;
            }
        }
        false
    }

    pub fn add (&mut self, value: ValueType) {
        match self.root.clone() {
            None => { 
                self.root = Some(Rc::new(RefCell::new(BNode{value, left: None, right: None})));
            }
            Some(node) => {
                let mut cur = node;
                loop {
                    if value < cur.borrow().value {
                        if cur.borrow().left.is_some() {
                            let clone = cur.borrow().left.as_ref().unwrap().clone();
                            cur = clone;
                        } else {
                            cur.borrow_mut().left = Some(Rc::new(RefCell::new(BNode { value: value, left: None, right: None })));
                            return;
                        }
                     } else if value > cur.borrow().value {
                        if cur.borrow().right.is_some() {
                            let clone = cur.borrow().right.as_ref().unwrap().clone();
                            cur = clone;
                        } else {
                            cur.borrow_mut().right = Some(Rc::new(RefCell::new(BNode { value: value, left: None, right: None })));
                            return;
                        }
                     } else {
                        return;
                    }
               }
            }
        }
    }

    pub fn remove(&mut self, value: ValueType) -> bool {
        let mut cur = self.root.clone();
        let mut parent: MaybeBoxedNode<ValueType> = None;
        let mut lparent = false;
        let mut rparent = false;
        loop {
            match cur.as_ref() {
                None => {
                    return false;
                }
                Some(cur_boxed) => {
                    if cur_boxed.borrow().value == value {
                        let left_child = cur_boxed.borrow_mut().left.take();
                        let right_child = cur_boxed.borrow_mut().right.take();
                        let merged = 
                            if left_child.is_none() {
                                right_child
                            } else if right_child.is_none() {
                                left_child
                            } else {
                                Self::bubble_smallest(right_child)
                            };
                        if lparent {
                            if let Some(ref parent_node) = parent {
                                parent_node.borrow_mut().left = merged;
                            } else { panic!(); }
                        } else if rparent {
                            if let Some(ref parent_node) = parent {
                                parent_node.borrow_mut().right = merged;
                            } else { panic!(); }
                        } else {
                            self.root = None;
                        }
                        return true;
                    } else if cur_boxed.borrow().value > value {
                        lparent = true;
                        rparent = false;
                        let clone = cur_boxed.borrow().left.clone();
                        parent = cur;
                        cur = clone;
                    } else if cur_boxed.borrow().value < value {
                        lparent = false;
                        rparent = true;
                        let clone = cur_boxed.borrow().right.clone();
                        parent = cur;
                        cur = clone;
                    }
                }
            }
        }
    }

    fn bubble_smallest(subtree: MaybeBoxedNode<ValueType>) -> MaybeBoxedNode<ValueType> {
        if subtree.is_none() || subtree.as_ref().unwrap().borrow().left.is_none() {
            return subtree;
        }
        let mut cur = subtree.clone();
        let mut parent = None;
        while cur.as_ref().unwrap().borrow().left.is_some() {
            let clone = cur.as_ref().unwrap().borrow().left.clone();
            parent = cur;
            cur = clone;
        }

        let new_root = parent.unwrap().borrow_mut().left.take();
        new_root.as_ref().unwrap().borrow_mut().right = subtree;
        return new_root;
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};
    use crate::{BNode, BST};

    #[test]
    fn search_in_empty() {
        let bst = BST::new();
        assert!(!bst.contains(1));
    }

    #[test]
    fn remove_from_empty() {
        let mut bst = BST::new();
        assert!(!bst.remove(1));
    }

    #[test]
    fn add_to_empty() {
        let mut bst = BST::new();
        bst.add(1);
        assert!(bst.contains(1));
    }

    #[test]
    fn add_then_remove() {
        let n1 = Rc::new(RefCell::new(BNode{value: 1, left: None, right: None}));
        let n2 = Rc::new(RefCell::new(BNode{value: 3, left: None, right: None}));
        let p = Rc::new(RefCell::new(BNode{value: 2, left: Some(n1), right: Some(n2)}));
    
        let mut tree = BST { root: Some(p) };
        assert_eq!(false, tree.contains(4));
        tree.add(4);
        assert_eq!(true, tree.contains(4));
        assert_eq!(true, tree.remove(4));
        assert_eq!(false, tree.contains(4));
    }
}
