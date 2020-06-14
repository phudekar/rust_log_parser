use super::super::log_message::LogMessage;

#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode {
    Leaf,
    Node {
        left: Option<Box<MessageTree>>,
        right: Option<Box<MessageTree>>,
    },
}

impl TreeNode {
    pub fn new(left: MessageTree, right: MessageTree) -> Self {
        TreeNode::Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn with_left(tree: MessageTree) -> Self {
        TreeNode::Node {
            left: Some(Box::new(tree)),
            right: None,
        }
    }

    fn with_right(tree: MessageTree) -> Self {
        TreeNode::Node {
            right: Some(Box::new(tree)),
            left: None,
        }
    }

    fn update_left(&self, tree: MessageTree) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_left(tree),
            TreeNode::Node { left: _, right } => TreeNode::Node {
                left: Some(Box::new(tree)),
                right: right.clone(),
            },
        }
    }

    fn update_right(&self, tree: MessageTree) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_right(tree),
            TreeNode::Node { left, right: _ } => TreeNode::Node {
                right: Some(Box::new(tree)),
                left: left.clone(),
            },
        }
    }
}

impl std::convert::AsRef<TreeNode> for TreeNode {
    fn as_ref(&self) -> &TreeNode {
        &self
    }
}

#[derive(Debug, PartialEq)]
pub struct MessageTree {
    pub node_type: TreeNode,
    pub message: LogMessage,
}

impl std::clone::Clone for MessageTree {
    fn clone(&self) -> Self {
        MessageTree {
            message: self.message.clone(),
            node_type: match &self.node_type {
                TreeNode::Leaf => TreeNode::Leaf,
                TreeNode::Node { left, right } => TreeNode::Node {
                    left: left.clone(),
                    right: right.clone(),
                },
            },
        }
    }
}

impl MessageTree {
    pub fn from(message: LogMessage) -> MessageTree {
        MessageTree {
            node_type: TreeNode::Leaf,
            message,
        }
    }

    pub fn build(messages: Vec<LogMessage>) -> Result<MessageTree, String> {
        if let Some((head, tail)) = messages.split_first() {
            let mut root = Self::from(head.clone());
            for message in tail {
                root = root.insert(message.clone());
            }
            Ok(root)
        } else {
            Err(String::from("Cannot construct tree from empty messages"))
        }
    }

    pub fn insert(&self, message: LogMessage) -> MessageTree {
        if message.timestamp <= self.message.timestamp {
            self.insert_left(message)
        } else {
            self.insert_right(message)
        }
    }

    pub fn insert_left(&self, message: LogMessage) -> MessageTree {
        return MessageTree {
            node_type: match self.node_type.clone() {
                TreeNode::Leaf => TreeNode::with_left(MessageTree::from(message)),
                TreeNode::Node { left, right: _ } => match left {
                    Some(tree) => self.node_type.update_left(tree.insert(message)),
                    _ => self.node_type.update_left(MessageTree::from(message)),
                },
            },
            message: self.message.clone(),
        };
    }

    pub fn insert_right(&self, message: LogMessage) -> MessageTree {
        return MessageTree {
            node_type: match self.node_type.clone() {
                TreeNode::Leaf => TreeNode::with_right(MessageTree::from(message)),
                TreeNode::Node { left: _, right } => match right {
                    Some(tree) => self.node_type.update_right(tree.insert(message)),
                    _ => self.node_type.update_right(MessageTree::from(message)),
                },
            },
            message: self.message.clone(),
        };
    }
}
