use crate::core::tree::ArenaTree;
use crate::core::diff::Diff;
use std::cell::RefCell;
use std::process::Child;
use std::rc::Rc;

type DiffTreeNode = Rc<RefCell<DiffTree>>;

struct DiffTree{
    internal_structure: ArenaTree<Diff>,
    parent: Option<DiffTreeNode>,
    children: Vec<DiffTreeNode>,
}

impl DiffTree{

    pub fn new(internal_structure: ArenaTree<Diff>) -> DiffTreeNode{
        Rc::new(RefCell::new(DiffTree {
            internal_structure,
            parent: None,
            children: Vec::new(),
        }
    ))}

    pub fn add_child(parent: &DiffTreeNode, internal_structure: ArenaTree<Diff>) -> DiffTreeNode{
        let child = DiffTree::new(internal_structure);
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(Rc::clone(&child));

        child
    }

    pub fn display(node: &DiffTreeNode, depth: usize){
        let indent = " ".repeat(depth*2);
        println!("{}Node: {:?}",indent,node.borrow().internal_structure);

        for child in &node.borrow().children {
            DiffTree::display(child, depth + 1);
        }
    }
}