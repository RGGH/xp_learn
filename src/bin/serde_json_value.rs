use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Serializing a custom struct into JSON
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserializing JSON back into a custom struct
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);

    // Using `Value` to handle dynamic JSON data
    let json_value: Value = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized as Value: {:?}", json_value);

    // Manipulating the `Value` type directly
    let mut dynamic_json = json!({
        "name": "Bob",
        "age": 25,
    });

    // Change values in the dynamic JSON object
    dynamic_json["age"] = json!(26);
    println!("Modified dynamic JSON: {:?}", dynamic_json);
}
