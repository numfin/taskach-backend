use crate::firestore::prelude::*;

pub fn split_parent_and_collection_id(path: &String) -> (String, String) {
    let mut path = path.split('/').collect::<Vec<&str>>();
    let collection_id = path.pop().unwrap();
    let path = if path.len() > 0 {
        format!("/{}", path.join("/"))
    } else {
        "".to_string()
    };
    (format!("{}{}", PARENT, path), collection_id.to_string())
}
