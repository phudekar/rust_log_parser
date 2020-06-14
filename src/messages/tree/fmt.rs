use std::fmt;

use super::message_tree::{MessageTree, TreeNode};

impl std::fmt::Display for MessageTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            TreeNode::Leaf => writeln!(f, "{}", self.message.clone()),
            TreeNode::Node { left, right } => {
                let _ = writeln!(f, "{}", self.message.clone());
                let _ = left.clone().map(|s| writeln!(f, "-- {}", &s));
                let _ = right.clone().map(|s| writeln!(f, "-- {}", &s));
                writeln!(f, "|")
            }
        }
    }
}
