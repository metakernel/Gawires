pub mod difftree;
pub mod hash;

use hash::{DiffHash, DiffHasher};

// Path: src\core\diff.rs

#[derive(PartialEq, Clone, Debug)]
pub struct Diff {
    pub version_hash: DiffHash,
    pub diff: DiffData,
    pub diff_type: DiffType,
}

#[derive(PartialEq,Clone, Debug)]
pub enum DiffData {
    Add(AddDiff),
    Remove(RemoveDiff),
    Modify(ModifyDiff),
}

#[derive(PartialEq,Clone, Debug)]
pub enum DiffType {
    /// The diff is a full diff, and should be applied to the entire asset. Serve normaly as base diff.
    Reference,
    /// The diff is an incremental diff, and should be applied to the asset's version graph to a specific reference.
    Incremental,
}


/// A diff that adds a new value to the asset.
#[derive(PartialEq, Clone, Debug)]
pub struct AddDiff {
    pub path: String,
    pub value: Vec<u8>,
}

/// A diff that removes a value from the asset.
#[derive(PartialEq, Clone, Debug)]
pub struct RemoveDiff {
    pub path: String,
}

/// A diff that modifies a value in the asset.
#[derive(PartialEq, Clone, Debug)]
pub struct ModifyDiff {
    pub path: String,
    pub value: Vec<u8>,
}