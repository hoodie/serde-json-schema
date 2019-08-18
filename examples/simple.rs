use serde_json_schema::*;
use std::convert::TryFrom;

fn main() {
    let raw_schema = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Product",
  "description": "A product from Acme's catalog",
  "type": "object",
  "properties": {
    "productId": {
      "description": "The unique identifier for a product",
      "type": "integer"
    }
  },
  "required": [ "productId" ]
}"#;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);
    println!("{:#?}", schema.draft_version());
}
