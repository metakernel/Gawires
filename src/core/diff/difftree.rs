use crate::core::tree::ArenaTree;
use crate::core::diff::Diff;

struct DiffTree{
    internal_structure: ArenaTree<Diff>,
    parent: Box<Option<DiffTree>>,
}