/// Represents a tree of nodes that use indexes instead of refs/pointers.
#[derive(Debug)]
pub struct ArenaTree<T>
where
    T: PartialEq + Clone,{
    /// The root node of the tree.
    pub arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + Clone
{
    fn node(&mut self, data: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.data == data {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, data));
        idx
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn edges(&self) -> usize {
        self.arena.iter().fold(0, |acc, node| acc + node.children.len())
    }

    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    pub fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
        // are we here?  If so, Some(0)
        if target == &self.arena[idx].data {
            return Some(0);
        }
    
        // If not, try all children
        for p in &self.arena[idx].children {
            if let Some(x) = self.depth_to_target(*p, &target) {
                return Some(1 + x);
            }
        }
        // If it cant be found, return None
        None
    }

    pub fn distance_between(&mut self, from: T, target: T) -> usize {
        // If it's not in the tree, this will add a new unconnected node
        // the final function will still return None
        let start_node = self.node(from);
        let mut ret = 0;
        // Start traversal
        let mut trav = &self.arena[start_node];
        // Explore all children, then hop up one
        while let Some(inner) = trav.parent {
            if let Some(x) =  self.depth_to_target(inner, &target) {
                ret += x;
                break;
            }
            trav = &self.arena[inner];
            ret += 1;
        }
        // don't go all the way to target, just orbit
        ret - 1
    }

    /// Inserts a new node.
    pub fn insert(&mut self, parent: T, child: T) {
        let parent_idx = self.node(parent);
        let child_idx = self.node(child);
        self.arena[parent_idx].children.push(child_idx);
    }
    
    /// Remove a node.
    pub fn remove_node(&mut self, node: &T) {
        if let Some(idx) = self.depth_to_target(0, node) {
            self.arena.remove(idx);
        }
    }
}

/// Represent a node inside a tree data structure
#[derive(Debug)]
pub struct Node<T> 
    where T: PartialEq + Clone{
    /// The index of the node
    pub idx: usize,
    /// The data of the node
    pub data: T,
    /// The parent node, if any
    pub parent: Option<usize>,
    /// The children of the node.
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq + Clone
{
    fn new(idx: usize, data: T) -> Self {
        Self {
            idx,
            data,
            parent: None,
            children: vec![],
        }
    }
}