# serde-json-schema

Minimal implementation of [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html) using [serde-json](https://github.com/serde-rs/json).

## Example

```rust
use serde_json_schema::Schema;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_json = r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://example.com/product.schema.json",
        "title": "Product",
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "price": { "type": "number", "exclusiveMinimum": 0 }
        },
        "required": ["name", "price"]
    }"#;

    let schema = Schema::try_from(schema_json)?;
    println!("{:#?}", schema);
    Ok(())
}
```

## JSON Schema 2020-12 Features

This crate supports the following JSON Schema 2020-12 features:

* `$schema` - Schema dialect identifier
* `$id` - Schema identifier
* `$anchor` - Plain-name fragment identifiers
* `$dynamicAnchor` / `$dynamicRef` - Dynamic referencing
* `$vocabulary` - Meta-schema vocabulary declarations
* `$comment` - Schema comments
* `$defs` - Schema definitions
* `prefixItems` - Tuple validation
* `items` - Additional items schema
* `dependentRequired` / `dependentSchemas` - Property dependencies
* `unevaluatedProperties` / `unevaluatedItems` - Unevaluated applicators
* `contains` with `minContains` / `maxContains` - Array containment validation

## Features/TODO

* [x] JSON Schema Core Type
* [x] JSON Schema 2020-12 support
* [ ] JSON Schema Validation (partial, possibly different crate or optional feature)
* [ ] Codegen (definitely different crate)
* [ ] RootSchema vs SubSchema handling (is that used often?)
* [ ] References
* [ ] Test Serialization
* [ ] Complete Feature List
* [ ] Detect enum
* [ ] Detect const

## License

serde-json-schema is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/serde-json-schema/issues) or comprehensive pull requests are welcome! 


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in serde-json-schema by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
