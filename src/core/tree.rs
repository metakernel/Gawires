/// Represent a node inside a tree data structure
pub struct Node<T,'a> {
    /// The data stored in the node.
    pub data: T,
    /// The left children of the node.
    pub children: &Vec<Box<&'a Node<T>>>,
}

/// Represents a tree of nodes.
pub struct Tree<T,'a> {
    /// The root node of the tree.
    pub root: Box<&'a Node<T>>,
}