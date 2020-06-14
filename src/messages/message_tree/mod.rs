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

impl TreeNode {
    fn new(left: MessageTree, right: MessageTree) -> Self {
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
fn should_insert_new_node_in_message_tree() {
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

    let updated_tree = tree.insert(msg2.clone());
    assert_eq!(
        updated_tree.node_type,
        TreeNode::Node {
            left: None,
            right: Some(Box::new(MessageTree::from(msg2)))
        }
    );
}

#[test]
fn should_insert_new_node_to_left_in_message_tree() {
    let msg1 = LogMessage {
        message_type: MessageType::Info,
        timestamp: 2,
        message: String::from("hello"),
    };
    let msg2 = LogMessage {
        message_type: MessageType::Warning,
        timestamp: 1,
        message: String::from("warn"),
    };
    let tree = MessageTree::from(msg1);
    assert_eq!(tree.node_type, TreeNode::Leaf);

    let updated_tree = tree.insert(msg2.clone());
    assert_eq!(
        updated_tree.node_type,
        TreeNode::Node {
            right: None,
            left: Some(Box::new(MessageTree::from(msg2)))
        }
    );
}

#[test]
fn should_insert_multiple_in_message_tree() {
    let msg1 = LogMessage {
        message_type: MessageType::Info,
        timestamp: 1,
        message: String::from("1"),
    };
    let msg2 = LogMessage {
        message_type: MessageType::Warning,
        timestamp: 2,
        message: String::from("2"),
    };
    let msg3 = LogMessage {
        message_type: MessageType::Info,
        timestamp: 3,
        message: String::from("3"),
    };
    let msg4 = LogMessage {
        message_type: MessageType::Warning,
        timestamp: 4,
        message: String::from("4"),
    };
    let msg5 = LogMessage {
        message_type: MessageType::Warning,
        timestamp: 5,
        message: String::from("5"),
    };
    let mut tree = MessageTree::from(msg4);
    tree = tree.insert(msg5.clone());
    tree = tree.insert(msg2.clone());
    tree = tree.insert(msg3.clone());
    tree = tree.insert(msg1.clone());

    println!("{}", tree);

    assert_eq!(
        tree.node_type,
        TreeNode::Node {
            right: Some(Box::new(MessageTree::from(msg5))),
            left: Some(Box::new(MessageTree {
                node_type: TreeNode::new(MessageTree::from(msg1), MessageTree::from(msg3)),
                message: msg2,
            }))
        }
    );
}
