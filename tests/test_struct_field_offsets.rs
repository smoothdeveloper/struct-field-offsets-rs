use struct_field_offsets::FieldOffsets;

#[derive(FieldOffsets)]
#[repr(C)]
struct Data {
    x: i32,
    y: i32,
    label: [u8;8]
}

#[test]
fn test_struct_field_offsets() {
    let offsets = Data::field_offsets();
    for (name,offset) in offsets {
        println!("{name}: {offset}");
    }
    assert_eq!(("x",0), offsets[0]);
    assert_eq!(("y",4), offsets[1]);
    assert_eq!(("label",8), offsets[2]);
}