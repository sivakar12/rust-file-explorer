use std::fs;
use std::path::{PathBuf};
use crate::Node; 

pub fn get_home_directory() -> Option<PathBuf> {
    match std::env::current_dir() {
        Ok(path) => Some(path),
        _ => None
    }
}
pub fn get_nodes_in_path(path: &PathBuf) -> Result<Vec<Node>, std::io::Error> {
    let read_dir_items = fs::read_dir(path)?;
    let mut results: Vec<Node> = Vec::new();
    for node in read_dir_items {
        let p = node?.path();
        let is_dir = fs::metadata(&p)?.is_dir();
        let node_struct = if is_dir { Node::Folder(p) } else { Node::File(p) };
        results.push(node_struct);
    }
    Ok(results)
}