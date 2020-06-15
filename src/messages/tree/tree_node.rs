#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode<T>
where
    T: Clone,
{
    Leaf,
    Node {
        left: Option<Box<T>>,
        right: Option<Box<T>>,
    },
}

impl<T> TreeNode<T>
where
    T: Clone,
{
    #[allow(dead_code)]
    pub fn new(left: T, right: T) -> Self {
        TreeNode::Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn with_left(tree: T) -> Self {
        TreeNode::Node {
            left: Some(Box::new(tree)),
            right: None,
        }
    }

    pub fn with_right(tree: T) -> Self {
        TreeNode::Node {
            right: Some(Box::new(tree)),
            left: None,
        }
    }

    pub fn update_left(&self, tree: T) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_left(tree),
            TreeNode::Node { left: _, right } => TreeNode::Node {
                left: Some(Box::new(tree)),
                right: right.clone(),
            },
        }
    }

    pub fn update_right(&self, tree: T) -> Self {
        match self {
            TreeNode::Leaf => TreeNode::with_right(tree),
            TreeNode::Node { left, right: _ } => TreeNode::Node {
                right: Some(Box::new(tree)),
                left: left.clone(),
            },
        }
    }
}
