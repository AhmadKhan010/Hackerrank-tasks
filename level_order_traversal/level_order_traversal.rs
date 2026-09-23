use std::io::{self, BufRead};
use std::collections::VecDeque;

pub struct Node {
    pub data: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(node: Option<Box<Node>>, data: i32) -> Option<Box<Node>> {
        if let Some(mut n) = node {
            if data <= n.data {
                n.left = Self::insert(n.left, data);
            } else {
                n.right = Self::insert(n.right, data);
            }
            Some(n)
        } else {
            Some(Box::new(Node::new(data)))
        }
    }
}

fn level_order(root: &Option<Box<Node>>) {
    if let Some(r) = root {
        let mut queue = VecDeque::new();
        queue.push_back(r); 
        
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            print!("{} ", current.data);
            
            if let Some(left_node) = &current.left {
                queue.push_back(left_node);
            }
            
            if let Some(right_node) = &current.right {
                queue.push_back(right_node);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let _size: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let data_str = stdin_iterator.next().unwrap().unwrap();
    
    let mut root: Option<Box<Node>> = None;
    
    for val in data_str.split_whitespace() {
        let data: i32 = val.parse().unwrap();
        root = Node::insert(root, data);
    }

    level_order(&root);
}
