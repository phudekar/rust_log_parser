use super::super::log_message::{LogMessage, MessageType};
use super::message_tree::{MessageTree, TreeNode};

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
