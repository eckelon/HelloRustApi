use nanoserde::{DeJson, SerJson};
use rand::prelude::*;

#[derive(SerJson, DeJson, Clone, Debug)]
pub struct Person {
    name: String,
    age: u8,
    id: u16,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        let id: u16 = random();
        Self { name, age, id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_person() {
        let person = Person::new("Person".to_string(), 29);
        assert_eq!(person.age, 29);
        assert_eq!(person.name, "Person");
        assert!(person.id >= u16::MIN && person.id <= u16::MAX);
    }
}
