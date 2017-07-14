extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn example() -> Result<(), Error> {
    let file = File::open("resources/person.json").expect("File not found");
    let p: Person = serde_json::from_reader(file)?;
    println!("{} is {} years old.", p.name, p.age);
    Ok(())
}

fn main() {
    example();
}
