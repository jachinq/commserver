use lib_json::object::JsonObject;

fn main() {
    let json = r#"{"name":"John Doe","age":18,"list":[1,2,4],"recur":"\"111\""}"#;
    let object = json.parse::<JsonObject>().unwrap();
    println!("{}", object.to_json_line());
}
