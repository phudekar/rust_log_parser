use super::tree_node::TreeNode;

#[derive(Debug, PartialEq)]
pub struct MessageTree<T>
where
    T: Clone + PartialOrd,
{
    pub node_type: TreeNode<MessageTree<T>>,
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

    fn insert_left(&self, message: &T) -> Self {
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

    fn insert_right(&self, message: &T) -> Self {
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
