use zson::{decode_json, encode_json};

#[derive(zson::Encodable)]
struct User {
    first_name: String,
    last_name: Option<String>,
    age: i64,
    pets: Vec<Pet>,
}

#[derive(zson::Encodable)]
struct Pet {
    name: String,
    species: String,
}

fn main() {
    let user = User {
        first_name: "John Doe".to_owned(),
        last_name: None,
        age: 25,
        pets: vec![
            Pet { name: "Dopey".to_owned(), species: "Cat".to_owned() },
            Pet { name: "Tinker".to_owned(), species: "Cat".to_owned() },
        ],
    };
    
    let json = encode_json(&user);
    println!("{}", json);

    let decoded = decode_json::<Vec<String>>("[\"John\", \"Doe\"]");
    println!("{:?}", decoded);
}
