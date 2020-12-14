# TernaryOption

Provides a Rust type and serialization/deserialization implementations for values that can be represented 
by 3 states: missing, present but null and present with a value. 

This can be useful when using JSON or other formats whereby the default in serde is to treat missing keys 
and null values as the same, deserializing them to `Option::None`. A similar problem exists when serializing
as you have to create your own 3-state enum and tag each field with `skip_serializing_if` in order to not
serialize a field.

This can be problematic with APIs where partial objects are provided and you don't know whether you need to 
set the value to null or if the key was missing from the incoming payload meaning the value should be left 
alone.

By using TernaryOption you are able to distinguish between null values and missing keys and get serde to 
behave correctly in these scenarios.

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use ternary_option::{TernaryOption::{self, *}, serde_ternary_fields};

#[serde_ternary_fields]
#[derive(Debug, Serialize, Deserialize)]
struct Thing {
    one: TernaryOption<u8>,
    two: TernaryOption<u8>,
    three: TernaryOption<u8>,
}

fn main() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "one": 1,
            "two": null,
        }
    ))
    .unwrap();

    assert_eq!(Present(Some(1)), thing.one);
    assert_eq!(Present(None), thing.two);
    assert_eq!(Missing, thing.three);
}
```
