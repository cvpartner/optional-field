use serde::{Deserialize, Serialize};
use serde_json::json;
use ternary_option::serde_ternary_fields;
use ternary_option::TernaryOption::{self, *};

#[serde_ternary_fields]
#[derive(Debug, Serialize, Deserialize)]
struct Thing {
    mandatory: u8,
    optional: Option<u8>,
    ternary: TernaryOption<u8>,
}

#[test]
fn deserialize_missing() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "optional": null,
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(None, thing.optional);
    assert_eq!(Missing, thing.ternary);
}

#[test]
fn deserialize_null() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "optional": 2,
            "ternary": null
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(Some(2), thing.optional);
    assert_eq!(Present(None), thing.ternary);
}

#[test]
fn deserialize_value() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "mandatory": 1,
            "optional": null,
            "ternary": 2
        }
    ))
    .unwrap();

    assert_eq!(1, thing.mandatory);
    assert_eq!(None, thing.optional);
    assert_eq!(Present(Some(2)), thing.ternary);
}

#[test]
fn serialize_value() {
    let thing = Thing {
        mandatory: 1,
        optional: None,
        ternary: Present(Some(2)),
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "optional": null,
                "ternary": 2
            }
        ),
        json
    );
}

#[test]
fn serialize_null() {
    let thing = Thing {
        mandatory: 1,
        optional: Some(2),
        ternary: Present(None),
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "optional": 2,
                "ternary": null
            }
        ),
        json
    );
}

#[test]
fn serialize_missing() {
    let thing = Thing {
        mandatory: 1,
        optional: None,
        ternary: Missing,
    };

    let json = serde_json::to_value(thing).unwrap();

    assert_eq!(
        json!(
            {
                "mandatory": 1,
                "optional": null,
            }
        ),
        json
    );
}
