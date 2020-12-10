extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Notepad {
    pub id: Option<i32>,
    pub slug: String,
    pub content: String
}