use born::{nested_macro, public_struct};
use serde::{Serialize, Deserialize}; 

public_struct!(
    #[derive(Deserialize, Serialize, Debug)]
    pub struct PersonBase {
        #[serde(rename = "person_email")]
        email: String,
    }
);

PersonBase!(pub struct Person); // You have to call it to use.
PersonBase!(pub struct PersonWithName {
    #[serde(rename = "person_name")]
    name: String,
});

// $cargo test -- --nocapture
#[test]
fn pass_public_struct_serde() {
    println!("pass_public_struct_serde");

    let email = "email@email.com";
    let name = "name";

    // You have to call it to use.
    let person = Person {
        email: email.into(),
    };

    assert_eq!(email, person.email);

    // Serialize the Person struct to JSON
    let person_serialized = serde_json::to_string(&person).unwrap();
    println!("Person Serialized JSON: {}", person_serialized);

    // Deserialize the JSON back into a Person struct
    let person_deserialized: Person = serde_json::from_str(&person_serialized).unwrap();
    println!("Person Deserialized Person: {:?}", person_deserialized);

    let person_with_name = PersonWithName {
        email: email.into(),
        name: name.into(),
    };

    assert_eq!(email, person_with_name.email);
    assert_eq!(name, person_with_name.name);

    let person_with_name_serialized = serde_json::to_string(&person_with_name).unwrap();
    println!("Person With Name Serialized JSON: {}", person_with_name_serialized);

    // Deserialize the JSON back into a Person struct
    let person_with_name_deserialized: PersonWithName = serde_json::from_str(&person_with_name_serialized).unwrap();
    println!("Person With Name Deserialized Person: {:?}", person_with_name_deserialized);



}

