use born::{nested_macro, private_enum};

use serde::{Serialize, Deserialize};

private_enum!(
    #[derive(Debug, Serialize, Deserialize)]
    enum PersonBase {
        #[serde(rename = "person_type_struct")] 
        TypeStruct { email: String, name: String },
        Type,
    }
);

PersonBase!(enum Person {
    TypeExtension
});

PersonBase!(enum PersonSerde {
    #[serde(rename = "person_type_extension")] 
    TypeExtensionWithSerde
});

// $cargo test -- --nocapture
#[test]
fn pass_private_enum_serde() {
    let email = "email@email.com";
    let name = "name";

    let person_typestruct: Person = Person::TypeStruct {
        email: email.into(),
        name: name.into(),
    };

    let serialized = serde_json::to_string(&person_typestruct).unwrap();
    println!("{}", serialized);
    // Output: {"person_type_a":{"name":"Alice","age":25}}
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
    // Output: TypeA { name: "Alice", age: 25 }

}
