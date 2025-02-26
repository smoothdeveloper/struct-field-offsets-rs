# struct-field-offsets

This crate provides a `field_offsets` member over `struct` declaration
that adorns the `FieldOffsets` macro.

For example, this can be used in FFI scenarios where asserting the offset
of each field among the various languages struct becomes a concern.

```rust
use struct_field_offsets::FieldOffsets;

// at declaration site
#[derive(FieldOffsets)]
#[repr(C)]
struct Data {
    x: i32,
    y: i32,
    label: [u8;8]
}

// in the code
let offsets = Data::field_offsets();
for (name,offset) in offsets {
    println!("field {name} offset is {offset}.");
}
// prints:
// > field x offset is 0.
// > field y offset is 4.
// > field label offset is 8.
```

In your Cargo.toml:
```toml
[dependencies]
struct-field-offsets = "*"
```

Similar crates:
* Accessing field pointers:
  * [const-field-offset](https://crates.io/crates/const-field-offset)
  * [field-offset](https://crates.io/crates/field-offset)
* Retrieving the field count: [const_field_count](https://crates.io/crates/const_field_count)