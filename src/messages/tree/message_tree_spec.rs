use super::super::log_message::{LogMessage, MessageType};
#[allow(unused_imports)]
use super::message_tree::MessageTree;
use super::tree_node::TreeNode;

#[allow(dead_code)]
struct TestData;

impl TestData {
    #[allow(dead_code)]
    fn messages() -> Vec<LogMessage> {
        vec![
            LogMessage {
                message_type: MessageType::Info,
                timestamp: 1,
                message: String::from("1"),
            },
            LogMessage {
                message_type: MessageType::Warning,
                timestamp: 2,
                message: String::from("2"),
            },
            LogMessage {
                message_type: MessageType::Info,
                timestamp: 3,
                message: String::from("3"),
            },
            LogMessage {
                message_type: MessageType::Warning,
                timestamp: 4,
                message: String::from("4"),
            },
            LogMessage {
                message_type: MessageType::Warning,
                timestamp: 5,
                message: String::from("5"),
            },
        ]
    }
    #[allow(dead_code)]
    fn message_at(index: usize) -> LogMessage {
        Self::messages().get(index).unwrap().clone()
    }
}

#[test]
fn should_create_message_tree() {
    let tree = MessageTree::from(&LogMessage {
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
    let tree = MessageTree::from(&msg1);
    assert_eq!(tree.node_type, TreeNode::Leaf);

    let updated_tree = tree.insert(&msg2);
    assert_eq!(
        updated_tree.node_type,
        TreeNode::Node {
            left: None,
            right: Some(Box::new(MessageTree::from(&msg2)))
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
    let tree = MessageTree::from(&msg1);
    assert_eq!(tree.node_type, TreeNode::Leaf);

    let updated_tree = tree.insert(&msg2);
    assert_eq!(
        updated_tree.node_type,
        TreeNode::Node {
            right: None,
            left: Some(Box::new(MessageTree::from(&msg2)))
        }
    );
}

#[test]
fn should_insert_multiple_in_message_tree() {
    let mut tree = MessageTree::from(&TestData::message_at(3));
    tree = tree.insert(&TestData::message_at(4));
    tree = tree.insert(&TestData::message_at(1));
    tree = tree.insert(&TestData::message_at(2));
    tree = tree.insert(&TestData::message_at(0));

    println!("{}", tree);

    assert_eq!(
        tree.node_type,
        TreeNode::Node {
            right: Some(Box::new(MessageTree::from(&TestData::message_at(4)))),
            left: Some(Box::new(MessageTree {
                node_type: TreeNode::new(
                    MessageTree::from(&TestData::message_at(0)),
                    MessageTree::from(&TestData::message_at(2))
                ),
                message: TestData::message_at(1),
            }))
        }
    );
}

#[test]
fn should_build_message_tree() {
    if let Ok(tree) = MessageTree::build(&vec![
        TestData::message_at(3),
        TestData::message_at(4),
        TestData::message_at(1),
        TestData::message_at(0),
        TestData::message_at(2),
    ]) {
        assert_eq!(
            tree.node_type,
            TreeNode::Node {
                right: Some(Box::new(MessageTree::from(&TestData::message_at(4)))),
                left: Some(Box::new(MessageTree {
                    node_type: TreeNode::new(
                        MessageTree::from(&TestData::message_at(0)),
                        MessageTree::from(&TestData::message_at(2))
                    ),
                    message: TestData::message_at(1),
                }))
            }
        );
    } else {
        panic!("Failed to build Message Tree")
    }
}

#[test]
fn should_return_ordered_message_tree() {
    let unorderd_messages = vec![
        TestData::message_at(3),
        TestData::message_at(4),
        TestData::message_at(1),
        TestData::message_at(0),
        TestData::message_at(2),
    ];
    let tree = MessageTree::build(&unorderd_messages).unwrap();
    let ordered_messages = MessageTree::in_order(&tree);
    assert_eq!(ordered_messages.get(0).unwrap().timestamp, 1);
    assert_eq!(ordered_messages.get(1).unwrap().timestamp, 2);
    assert_eq!(ordered_messages.get(2).unwrap().timestamp, 3);
    assert_eq!(ordered_messages.get(3).unwrap().timestamp, 4);
    assert_eq!(ordered_messages.get(4).unwrap().timestamp, 5);
}
