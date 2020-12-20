use crate::firestore::prelude::*;

pub fn split_parent_and_collection_id<'a>(path: &'a str) -> (String, String) {
    let mut path = path.split('/').collect::<Vec<&str>>();
    let collection_id = if let Some(v) = path.pop() { v } else { "" };
    let path = if path.len() > 0 {
        format!("/{}", path.join("/"))
    } else {
        "".to_string()
    };
    (format!("{}{}", PARENT, path), collection_id.to_string())
}
