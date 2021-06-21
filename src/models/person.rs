use nanoserde::{DeJson, SerJson};
use rand::prelude::*;

#[derive(SerJson, DeJson)]
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
    fn it_creates_a_person_struct() {
        let person = Person {
            name: "Name".to_string(),
            age: 76,
            id: 2345,
        };
        assert_eq!(person.name, "Name");
        assert_eq!(person.age, 76);
        assert_eq!(person.id, 2345);
    }
    #[test]
    fn it_creates_a_new_person() {
        let person = Person::new("Person".to_string(), 29);
        assert_eq!(person.age, 29);
        assert_eq!(person.name, "Person");
        assert!(person.id >= u16::MIN && person.id <= u16::MAX);
    }
    #[test]
    fn it_serializes_a_person() {
        let person = Person {
            name: "Name".to_string(),
            age: 76,
            id: 2345,
        };
        let person_json = SerJson::serialize_json(&person);
        assert_eq!(person_json, "{\"name\":\"Name\",\"age\":76,\"id\":2345}");
    }
    #[test]
    fn it_deserializes_a_person() {
        let person_json = "{\"name\":\"Name\",\"age\":76,\"id\":2345}";
        let person: Person = DeJson::deserialize_json(&person_json).unwrap();
        assert_eq!(person.name, "Name");
        assert_eq!(person.age, 76);
        assert_eq!(person.id, 2345);
    }
}
