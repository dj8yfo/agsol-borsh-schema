use super::TEST_DATA_DIRECTORY;

use borsh::{BorshSchema, BorshDeserialize, BorshSerialize};
use serde::Serialize;

use std::fs;
use std::io::Write;
use crate::layout::Layout;


type UnixTimestamp = i64;

pub type Amount = u64;

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Clone, Copy, Debug)]
pub struct OtherState {
    amount: Amount,
    timestamp: UnixTimestamp,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestData {
    test_struct_some: Vec<u8>,
    test_struct_none: Vec<u8>,
    tuple_struct: Vec<u8>,
}

type StatePool = Option<Vec<OtherState>>;

#[allow(dead_code)]
#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TestStruct {
    field_a: u64,
    field_b: u8,
    // #[alias(Option<Vec<OtherState>>)]
    field_c: StatePool,
    #[borsh_skip]
    skipped_field: Option<u32>,
}


#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TupleStruct(u8, pub i32, pub OtherState);


#[test]
fn generate_layout_from_this_file() {
    let container = <OtherState as BorshSchema>::schema_container();
    let other_state_l = Layout::from_borsh_container(container).unwrap();

    assert_eq!(other_state_l[0].name, "OtherState");

    let container = <TupleStruct as BorshSchema>::schema_container();
    let tuple_struct_l = Layout::from_borsh_container(container).unwrap();
    
    assert_eq!(tuple_struct_l[0].name, "TupleStruct");
    //
    let container = <TestStruct as BorshSchema>::schema_container();
    let test_struct_l = Layout::from_borsh_container(container).unwrap();

    assert_eq!(test_struct_l[0].name, "TestStruct");

    let test_struct_none = TestStruct {
        field_a: 45678910,
        field_b: 103,
        field_c: None,
        skipped_field: Some(323424),
    };

    let other_state_one = OtherState {
        amount: 1_000_000_000,
        timestamp: 1234567890,
    };

    let other_state_two = OtherState {
        amount: 2_000_000_000,
        timestamp: 1234567891,
    };

    let other_state_three = OtherState {
        amount: 3_000_000_000,
        timestamp: 1234567892,
    };

    let test_struct_some = TestStruct {
        field_a: 10,
        field_b: 113,
        field_c: Some(vec![other_state_one, other_state_two, other_state_three]),
        skipped_field: None,
    };

    let tuple_struct = TupleStruct(10, -2, other_state_one);

    let test_data = TestData {
        test_struct_some: test_struct_some.try_to_vec().unwrap(),
        test_struct_none: test_struct_none.try_to_vec().unwrap(),
        tuple_struct: tuple_struct.try_to_vec().unwrap(),
    };

    fs::create_dir_all(TEST_DATA_DIRECTORY).unwrap();
    let mut file =
        fs::File::create(String::from(TEST_DATA_DIRECTORY) + "/test_structs.json").unwrap();
    write!(file, "{}", serde_json::to_string(&test_data).unwrap()).unwrap();
}

