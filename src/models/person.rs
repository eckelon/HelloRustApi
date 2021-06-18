use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Clone, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
