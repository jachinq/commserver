use lib_json::{list::JsonList, object::JsonObject};

fn main() {
    // Parse a JSON string into a JsonObject
    let object = parse_object();
    println!("Object: {:?}", object);

    // Convert the JsonObject back to a JSON string
    println!("JSON: {}", object.to_json());

    // Accessing values in the JsonObject
    println!("Name: {}", object.get_str("name").unwrap());
    println!("Age: {}", object.get_i32("age").unwrap());
    println!("City: {}", object.get_str("city").unwrap());

    // Adding values to the JsonObject
    let mut object = JsonObject::new();
    object.set_str("Name", "Richard Roe");
    object.set_i32("Age", 28);
    object.set_str("City", "San Francisco");
    object.set_list("phones", JsonList::new(&["012-345-6789", "555-555-5555"]));
    object.set_list("check", JsonList::new(&[1, 2, 3, 4, 5]));
    println!("Updated Object: {:?}", object);
}

fn parse_object() -> JsonObject {
    let json_str = r#"{"name": "John Doe", "age": 30, "city": "New York"}"#;
    json_str.parse::<JsonObject>().unwrap()
}
