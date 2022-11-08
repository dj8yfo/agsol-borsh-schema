use crate::{generate_output, layout::Layout};
use super::borsh_structs::{OtherState, TupleStruct, TestStruct};
use borsh::BorshSchema;

#[test]
fn generate_output_from_test_directory() {
    let mut layouts = Vec::new();

    let other_state_l =
        Layout::from_borsh_container(<OtherState as BorshSchema>::schema_container())
            .unwrap();

    let tuple_struct_layout =
        Layout::from_borsh_container(<TupleStruct as BorshSchema>::schema_container())
            .unwrap();

    let test_struct_layout =
        Layout::from_borsh_container(<TestStruct as BorshSchema>::schema_container())
            .unwrap();
    layouts.push(other_state_l);
    layouts.push(tuple_struct_layout);
    layouts.push(test_struct_layout);
    generate_output(&layouts, "test-rs-output-ts-input").unwrap();
}
