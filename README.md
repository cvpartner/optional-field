# Optional Field

> Want to write Rust for a living? CV Partner is hiring Rust developers in London, Oslo, Copenhagen and Stockholm. [See the careers page](https://www.cvpartner.com/careers)

Provides a Rust type and serialization/deserialization implementations for values that can be represented
by 3 states: missing, present but null and present with a value.

```rust
pub enum Field<T> {
    Missing,
    Present(Option<T>),
}

```

This can be useful when using JSON or other formats whereby the default in serde is to treat missing keys
and null values as the same, deserializing them to `Option::None`. A similar problem exists when serializing
as you have to create your own 3-state enum and tag each field with `skip_serializing_if` in order to not
serialize a field.

This can be problematic with APIs where partial objects or diffs are provided and you don't know whether you need to
set the value to null or not update value should be left alone.

By using `Field` you are able to distinguish between null values and missing keys and get serde to
behave correctly in these scenarios.

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use optional_field::{Field, serde_optional_fields};

#[serde_optional_fields]
#[derive(Debug, Serialize, Deserialize)]
struct Thing {
    one: Field<u8>,
    two: Field<u8>,
    three: Field<u8>,
}

fn main() {
    let thing = serde_json::from_value::<Thing>(json!(
        {
            "one": 1,
            "two": null,
        }
    ))
    .unwrap();

    assert_eq!(Field::Present(Some(1)), thing.one);
    assert_eq!(Field::Present(None), thing.two);
    assert_eq!(Field::Missing, thing.three);
}
```

## Usage

[Field](src/lib.rs) implements many of the methods you are familiar with
on Option such as `map`, `unwrap`, `as_ref` etc. `Field` will return the value
from within the `Option` for these methods but also provides an equivalent set of methods for accessing the `Option` itself. These equivalent methods follow the
pattern of adding `_present` to the method name. For example, given `Present(Some(100))`, `unwrap()` will return `100` whereas `unwrap_present()` will return `Some(100)`.

```rust
use optional_field::Field;

struct Thing {
    one: Field<u8>,
    two: Field<u8>,
    three: Field<u8>,
}

fn main() {
    let num_field = Field::Present(Some(100));
    // Calling map gets the value out of the Option within Present
    assert_eq!(200, num_field.map(|n| n * 2));
    // Calling map_present gets the option out of Present
    assert_eq!(false, num_field.map_present(|opt| opt.is_none()));
}
```

## Features

By default `optional-field` has serde and the serde macro as dependencies. If you
wish to use `optional-field` without pulling in serde you can set `default-features` to false.

```toml
[dependencies]
optional-field = { version = "0.1.4", default-features = false }
```

## License

MIT license ([LICENSE.txt](LICENSE.txt) or http://opensource.org/licenses/MIT)
