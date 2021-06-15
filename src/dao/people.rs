use crate::models::person::Person;
use std::env;
use std::fs;

fn from_file(file_name: String) -> Result<Vec<Person>, String> {
   fs::read_to_string(file_name)
      .map_err(|e| e.to_string())
      .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}

fn get_people_result() -> Result<Vec<Person>, String> {
   let datasource_key = "PEOPLE_API_DATASOURCE_FILE";
   let datasource = match env::var(datasource_key) {
      Ok(variable) => variable,
      Err(_) => {
         log::warn!("{} env variable has not been customized!", datasource_key);
         "./examples/people_list.json".to_string()
      }
   };
   from_file(datasource)
}

fn reply_result(result: Result<Vec<Person>, String>) -> Vec<Person> {
   match result {
      Ok(result) => result,
      Err(error) => {
         log::error!("There was an error getting the People list! {}", error);
         Vec::new()
      }
   }
}

pub fn get_people() -> Vec<Person> {
   let people_result = get_people_result();
   reply_result(people_result)
}

#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test_get_people_with_default_mock_data() {
      let people_example = fs::read_to_string("./examples/people_list.json").unwrap();
      let people = get_people();
      assert_eq!(
         people_example.trim(),
         serde_json::to_string(&people).unwrap()
      );
   }

   #[test]
   fn test_get_people_with_successful_mock_data() {
      env::set_var("PEOPLE_API_DATASOURCE_FILE", "./examples/people_list.json");
      let people_example = fs::read_to_string("./examples/people_list.json").unwrap();
      let people = get_people();
      assert_eq!(
         people_example.trim(),
         serde_json::to_string(&people).unwrap()
      );
   }
   #[test]
   fn test_get_people_with_wrong_mock_data() {
      env::set_var("PEOPLE_API_DATASOURCE_FILE", "wrong_file.fake");
      let people_example = "[]";
      let people = get_people();
      assert_eq!(people_example, serde_json::to_string(&people).unwrap());
   }
}
