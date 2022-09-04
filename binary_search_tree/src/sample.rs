use std::rc::Rc;
use std::cell::RefCell;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: MaybeNode,
  pub right: MaybeNode,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
   }
 }
 
//impl Solution {
  pub fn is_same_tree(p: MaybeNode, q: MaybeNode) -> bool {
      let mut stack = vec![];
      stack.push((p, q));
      while !stack.is_empty() {
          // unwrapping is safe because of our while condition
          let pair = stack.pop().unwrap();
          match pair {
              (Some(p), Some(q)) if p == q => {
                  stack.push((p.borrow().left.clone(), q.borrow().left.clone()));
                  stack
                      .push((p.borrow().right.clone(), q.borrow().right.clone()));
              }
              (None, None) => {}
              _ => {
                  return false;
              }
          }
      }
      true
  }
//}