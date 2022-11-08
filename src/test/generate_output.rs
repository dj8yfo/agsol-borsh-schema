use crate::{generate_output, layout::Layout};
use super::borsh_structs::OtherState;
use borsh::BorshSchema;

#[test]
fn generate_output_from_test_directory() {
    let mut layouts = Vec::new();

    let other_state_l =
        Layout::from_borsh_container(<OtherState as BorshSchema>::schema_container())
            .unwrap();
    layouts.push(other_state_l);
    generate_output(&layouts, "test-rs-output-ts-input").unwrap();
}
