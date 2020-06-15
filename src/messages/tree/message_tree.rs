#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode<T>
where
    T: Clone + PartialOrd,
{
    Leaf,
    Node {
        left: Option<Box<MessageTree<T>>>,
        right: Option<Box<MessageTree<T>>>,
    },
}

impl<T> TreeNode<T>
where
    T: Clone + PartialOrd,
{
    #[allow(dead_code)]
    pub fn new(left: MessageTree<T>, right: MessageTree<T>) -> Self {
        TreeNode::Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn with_left(tree: MessageTree<T>) -> Self {
        TreeNode::Node {
            left: Some(Box::new(tree)),
            right: None,
        }
    }

    fn with_right(tree: MessageTree<T>) -> Self {
        TreeNode::Node {
            right: Some(Box::new(tree)),
            left: None,
        }
    }

    fn update_left(&self, tree: MessageTree<T>) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_left(tree),
            TreeNode::Node { left: _, right } => TreeNode::Node {
                left: Some(Box::new(tree)),
                right: right.clone(),
            },
        }
    }

    fn update_right(&self, tree: MessageTree<T>) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_right(tree),
            TreeNode::Node { left, right: _ } => TreeNode::Node {
                right: Some(Box::new(tree)),
                left: left.clone(),
            },
        }
    }
}

impl<T> std::convert::AsRef<TreeNode<T>> for TreeNode<T>
where
    T: Clone + PartialOrd,
{
    fn as_ref(&self) -> &TreeNode<T> {
        &self
    }
}

#[derive(Debug, PartialEq)]
pub struct MessageTree<T>
where
    T: Clone + PartialOrd,
{
    pub node_type: TreeNode<T>,
    pub message: T,
}

impl<T> std::clone::Clone for MessageTree<T>
where
    T: Clone + PartialOrd,
{
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

impl<T> MessageTree<T>
where
    T: Clone + PartialOrd,
{
    pub fn from(message: &T) -> MessageTree<T> {
        MessageTree {
            node_type: TreeNode::Leaf,
            message: message.clone(),
        }
    }

    pub fn build(messages: &Vec<T>) -> Result<Self, String> {
        if let Some((head, tail)) = messages.split_first() {
            let mut root = Self::from(head);
            for message in tail {
                root = root.insert(message);
            }
            Ok(root)
        } else {
            Err(String::from("Cannot construct tree from empty messages"))
        }
    }

    pub fn insert(&self, message: &T) -> Self {
        if message <= &self.message {
            self.insert_left(&message)
        } else {
            self.insert_right(&message)
        }
    }

    pub fn insert_left(&self, message: &T) -> Self {
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

    pub fn insert_right(&self, message: &T) -> Self {
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

    pub fn in_order(&self) -> Vec<&T> {
        match &self.node_type {
            TreeNode::Leaf => vec![&self.message],
            TreeNode::Node { left, right } => {
                let mut ordered: Vec<&T> = vec![];
                if let Some(left_nodes) = left {
                    ordered.append(MessageTree::in_order(&left_nodes).as_mut());
                }
                ordered.push(&self.message);
                if let Some(right_nodes) = right {
                    ordered.append(MessageTree::in_order(&right_nodes).as_mut());
                }
                ordered
            }
        }
    }
}
