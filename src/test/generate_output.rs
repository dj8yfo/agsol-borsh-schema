use super::{
    borsh_enums::{RandomStruct, TestEnum},
    borsh_structs::{OtherState, TestStruct, TupleStruct}, borsh_hashmap::HashMapWrapper,
};
use crate::{generate_output, layout::Layout};
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

    let random_struct_l =
        Layout::from_borsh_container(<RandomStruct as BorshSchema>::schema_container())
            .unwrap();
    let test_enum_l =
        Layout::from_borsh_container(<TestEnum as BorshSchema>::schema_container())
            .unwrap();
    let hash_map_wrapper =
        Layout::from_borsh_container(<HashMapWrapper as BorshSchema>::schema_container())
            .unwrap();

    layouts.extend(other_state_l);
    layouts.extend(tuple_struct_layout);
    layouts.extend(test_struct_layout);
    layouts.extend(random_struct_l);
    layouts.extend(test_enum_l);
    layouts.extend(hash_map_wrapper);
    generate_output(&layouts, "test-rs-output-ts-input").unwrap();
}
