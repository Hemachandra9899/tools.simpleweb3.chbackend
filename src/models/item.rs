use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataFile {
    pub items: Vec<Item>,
}

impl DataFile {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewItem {
    pub id: String,
    pub name: String,
}
