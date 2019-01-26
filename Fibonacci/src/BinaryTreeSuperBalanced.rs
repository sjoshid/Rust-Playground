#[derive(PartialEq)]
enum BinaryTreeNodeType {
    NonLeaf,
    Leaf,
}

struct BinaryTreeNode {
    value: usize,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
    type_of_node: BinaryTreeNodeType,
}

fn is_binary_tree_super_node(node: &Box<BinaryTreeNode>, mut depth: usize) {
    depth = depth + 1;
    let mut left_empty = false;
    let mut right_empty = false;
    match node.left_child {
        Some(ref lc) => is_binary_tree_super_node(lc, depth),
        None => left_empty = true,
    }

    match node.right_child {
        Some(ref rc) => is_binary_tree_super_node(rc, depth),
        None => right_empty = true,
    }

    if left_empty && right_empty {

    }
}

fn buildTree() {
    /*let left_child = BinaryTreeNode {
        value: 1,
        left_child: Box::new(),
    }*/
}