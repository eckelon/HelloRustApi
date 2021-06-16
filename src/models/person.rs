use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
