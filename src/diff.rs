//! Create a diff between 2 strings
//! Serialize and deserialize diff
use similar::{ChangeTag, TextDiff};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Diff {
    op: String,
    value: String,
    index: usize,
}

pub fn create_diff() {

}

fn serialize() {

}

fn deserialize() {

}
