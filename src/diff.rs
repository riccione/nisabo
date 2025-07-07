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

fn create_diff(s1: &str, s2: &str) -> Vec<Diff> {
    let diff = TextDiff::configure()
        .diff_lines(
            s1, 
            s2);
    
    let mut changes = Vec::new();
    let mut curr_index = 0;

    for c in diff.iter_all_changes() {
        match c.tag() {
            ChangeTag::Equal => {
                curr_index += c.to_string().lines().count();
            },
            ChangeTag::Delete => changes.push(
                Diff {
                    op:  "-".to_string(),
                    value: c.to_string(),
                    index: curr_index,
                }),
            ChangeTag::Insert => {
                changes.push(
                    Diff {
                        op:  "+".to_string(),
                        value: c.to_string(),
                        index: curr_index,
                    });
                curr_index += c.to_string().lines().count();
            }
        }
    }

    changes
}

fn serialize(v: Vec<Diff>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&v)
}

fn deserialize(j: String) -> Result<Vec<Diff>, serde_json::Error> {
    serde_json::from_str(&j)
}

/// s1 - current string
/// v - diff
/// restores changes from the closest Diff, 
/// to handle restoring changes from deeper Diff, need to run it in a loop.
/// TODO: handle UI, run it in a background thread, as an idea - to simplify
/// add each 10 or 5 diffs a full copy?
fn restore_from_diff(s1: &str, v: &Vec<Diff>) -> String {
    let mut lines: Vec<String> = s1
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut changes_rev = v.to_vec();
    changes_rev.reverse();

    for c in changes_rev {
        match c.op.as_str() {
            "+" => {
                // Remove inserted lines from s1
                let num_lines = c.value.lines().count();
                if c.index + num_lines <= lines.len() {
                    lines.splice(c.index..c.index + num_lines, std::iter::empty());
                }
            },
            "-" => {
                // Reinsert deleted lines into s1
                let deleted_lines: Vec<String> = c.value
                    .lines()
                    .map(|x| x.to_string())
                    .collect();
                if c.index <= lines.len() {
                    lines.splice(c.index..c.index, deleted_lines);
                }
            },
            _ => {}
        }
    }

    lines.join("\n")
}

pub fn get_diff_json(s1: &str, s2: &str) -> String {
    let diff = create_diff(s1, s2);
    let json = match serialize(diff) {
        Ok(x) => x,
        Err(e) => {
            format!("Error: serialization failed {e}")
        }
    };
    json
}
