extern crate permission_test;
extern crate serde_json;

use permission_test::json_types::{
    Point,
    Permission,
};

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized Point = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized Point = {:?}", deserialized);

    let test_serialized = "{\"domain\":\"domain1\",\"actions\":[\"action1\",\"action2\",\"action3\"],\"targets\":[\"*\"]}";
    let test_deserialized_perm: Permission<String> = serde_json::from_str(&test_serialized).unwrap();
    println!("deserialized Test Permission = {:?}", test_deserialized_perm);

}
