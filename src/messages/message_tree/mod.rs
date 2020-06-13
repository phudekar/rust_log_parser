use super::log_message::LogMessage;
use super::log_message::MessageType;

#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode {
    Leaf,
    Node {
        left: Option<Box<MessageTree>>,
        right: Option<Box<MessageTree>>,
    },
}

impl std::convert::AsRef<TreeNode> for TreeNode {
    fn as_ref(&self) -> &TreeNode {
        &self
    }
}

#[derive(Debug, PartialEq)]
pub struct MessageTree {
    node_type: TreeNode,
    message: LogMessage,
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
                TreeNode::Leaf => TreeNode::Node {
                    left: Some(Box::new(MessageTree::from(message))),
                    right: None,
                },
                TreeNode::Node { left, right } => match left {
                    Some(tree) => TreeNode::Node {
                        left: Some(Box::new(tree.insert(message))),
                        right: right.clone(),
                    },
                    _ => TreeNode::Node {
                        left: Some(Box::new(MessageTree::from(message))),
                        right: right.clone(),
                    },
                },
            },
            message: self.message.clone(),
        };
    }

    pub fn insert_right(&self, message: LogMessage) -> MessageTree {
        return MessageTree {
            node_type: match self.node_type.clone() {
                TreeNode::Leaf => TreeNode::Node {
                    right: Some(Box::new(MessageTree::from(message))),
                    left: None,
                },
                TreeNode::Node { left, right } => match right {
                    Some(tree) => TreeNode::Node {
                        right: Some(Box::new(tree.insert(message))),
                        left: left.clone(),
                    },
                    _ => TreeNode::Node {
                        right: Some(Box::new(MessageTree::from(message))),
                        left: left.clone(),
                    },
                },
            },
            message: self.message.clone(),
        };
    }
}

#[test]
fn should_create_message_tree() {
    let tree = MessageTree::from(LogMessage {
        message_type: MessageType::Info,
        timestamp: 2,
        message: String::from("hello"),
    });
    assert_eq!(tree.node_type, TreeNode::Leaf)
}

#[test]
fn should_inser_new_node_in_message_tree() {
    let msg1 = LogMessage {
        message_type: MessageType::Info,
        timestamp: 2,
        message: String::from("hello"),
    };
    let msg2 = LogMessage {
        message_type: MessageType::Warning,
        timestamp: 3,
        message: String::from("warn"),
    };
    let tree = MessageTree::from(msg1);
    assert_eq!(tree.node_type, TreeNode::Leaf);

    tree.insert(msg2.clone());
    assert_eq!(
        tree.node_type,
        TreeNode::Node {
            left: None,
            right: Some(Box::new(MessageTree::from(msg2)))
        }
    );
}
