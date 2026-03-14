use zson::{Coder, CBOR, JSON};

#[derive(Debug, zson::Encodable, zson::Decodable)]
struct User {
    first_name: String,
    last_name: Option<String>,
    age: i64,
    pets: Vec<Pet>,
}

#[derive(Debug, zson::Encodable, zson::Decodable)]
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

    let cbor = CBOR::encode(&user);
    println!("{}", CBOR::hex_string(cbor));

    let as_cbor = JSON::convert::<CBOR>("{\
        \"first_name\": \"John Doe\",
        \"age\": 25,
        \"pets\": [{ \"name\": \"Woof\", \"species\": \"Dog\" }]
    }").unwrap();
    println!("{:#?}", CBOR::hex_string(as_cbor));

    let mixed_content = vec![
        zson::Value::String("Mixed".to_owned()),
        zson::Value::Number(123),
        zson::Value::String("Types".to_owned()),
        zson::Value::None,
    ];
    let mixed_content_json = JSON::encode(&mixed_content);
    let mixed_content_decoded = JSON::decode::<Vec<zson::Value>>(&mixed_content_json);
    println!("{:#?}", mixed_content_decoded);
}
