use super::{
    borsh_enums::{RandomStruct, TestEnum},
    borsh_hashmap::HashMapWrapper,
    borsh_structs::{OtherState, TestStruct, TupleStruct},
};
use crate::{construct_layouts, generate_output, layout::Layout};
use borsh::BorshSchema;

#[test]
fn generate_output_from_test_directory() {
    let layouts = construct_layouts!(
        OtherState,
        TupleStruct,
        TestStruct,
        RandomStruct,
        TestEnum,
        HashMapWrapper
    );
    generate_output(&layouts, "test-rs-output-ts-input").unwrap();
}
