use serde_json_schema::*;

#[test]
fn address_example() {
    let raw = include_str!("./fixtures/address.schema.json");
    let schema: Schema = serde_json::from_str(raw).unwrap();
    println!("{:#?}", schema);
}

#[test]
fn calendar_example() {
    let raw = include_str!("./fixtures/calendar.schema.json");
    let schema: Schema = serde_json::from_str(raw).unwrap();
    println!("{:#?}", schema);
}

#[test]
fn card_example() {
    let raw = include_str!("./fixtures/card.schema.json");
    let schema: Schema = serde_json::from_str(raw).unwrap();
    println!("{:#?}", schema);
}

#[test]
fn draft_version() {
    let schema: Schema = serde_json::from_str(include_str!("./fixtures/card.schema.json")).unwrap();
    println!("{:#?}", schema.draft_version());
    assert_eq!(schema.draft_version(), Some("draft-07"))
}

#[test]
fn green_door_example() {
    let schema: Schema =
        serde_json::from_str(include_str!("./fixtures/green_door.schema.json")).unwrap();
    let json_green_door: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/green_door.json")).unwrap();
    println!("{:#?}", schema);
    schema.validate(&json_green_door).unwrap();
}
