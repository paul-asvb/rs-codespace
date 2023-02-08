use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum City {
    Berlin,
    Paris,
}

#[derive(Serialize, Deserialize, Debug)]
struct CityComment {
    city: City,
    comment: String,
}

type TypeOne = Option<Vec<CityComment>>;

type TypeTwo = Option<HashMap<City, String>>;

fn main() {
    let type_one = r#"
        {
            "Berlin": "bla"
            "Paris": "bla2"
        },
    
      "#;
    let type_two = r#"
      [{
        "city": "Berlin",
        "comment": "bla"
      },{
        "city": "Paris",
        "comment": "bla2"
      }]"#;

    // let v: TypeOne = serde_json::from_str(type_one).unwrap();
    // println!("{:#?}", v);

    let v: TypeTwo = serde_json::from_str(type_two).unwrap();
    println!("{:#?}", v);
}
