/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}


impl<T> TreeNode<T>
where
    T: Ord + Debug,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.value == value || self.search_node(node, value),
            None => false,
        }
    }

    fn search_node(&self, node: &TreeNode<T>, value: T) -> bool {
        if value < node.value {
            if let Some(ref left) = node.left {
                return left.value == value || self.search_node(left, value);
            }
        } else if value > node.value {
            if let Some(ref right) = node.right {
                return right.value == value || self.search_node(right, value);
            }
        }
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


