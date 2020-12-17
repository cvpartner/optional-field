use optional_field::serde_optional_fields;
use optional_field::Field::{self, *};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[serde_optional_fields]
#[derive(Debug, Serialize, Deserialize)]
struct Thing {
    mandatory: u8,
    option: Option<u8>,
    field: Field<u8>,
}

#[test]
fn deserialize_missing() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "option": null,
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(None, thing.option);
    assert_eq!(Missing, thing.field);
}

#[test]
fn deserialize_null() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "option": 2,
            "field": null
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(Some(2), thing.option);
    assert_eq!(Present(None), thing.field);
}

#[test]
fn deserialize_value() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "option": null,
            "field": 2
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(None, thing.option);
    assert_eq!(Present(Some(2)), thing.field);
}

#[test]
fn serialize_value() {
    let thing = Thing {
        mandatory: 1,
        option: None,
        field: Present(Some(2)),
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "option": null,
                "field": 2
            }
        ),
        json
    );
}

#[test]
fn serialize_null() {
    let thing = Thing {
        mandatory: 1,
        option: Some(2),
        field: Present(None),
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "option": 2,
                "field": null
            }
        ),
        json
    );
}

#[test]
fn serialize_missing() {
    let thing = Thing {
        mandatory: 1,
        option: None,
        field: Missing,
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "option": null,
            }
        ),
        json
    );
}
