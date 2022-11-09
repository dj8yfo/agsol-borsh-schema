use super::TEST_DATA_DIRECTORY;
use crate::layout::Layout;

use borsh::{BorshSchema, BorshDeserialize, BorshSerialize};
use serde::Serialize;
use solana_program::pubkey::Pubkey;

use std::fs;
use std::io::Write;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestData {
    enum_variant_a: Vec<u8>,
    enum_variant_b: Vec<u8>,
    enum_variant_c: Vec<u8>,
    enum_variant_d: Vec<u8>,
    enum_variant_e: Vec<u8>,
    enum_variant_f: Vec<u8>,
    enum_variant_g: Vec<u8>,
}

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct RandomStruct {
    field_a: String,
    field_b: Option<[u8; 4]>,
}

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum TestEnum {
    VariantA,
    VariantB,
    VariantC(u64),
    VariantD(Option<Pubkey>),
    VariantE(Option<u8>),
    VariantF(RandomStruct),
    VariantG {
        #[allow(dead_code)]
        hello: Vec<u8>,
        #[allow(dead_code)]
        bello: [Pubkey; 3],
        #[allow(dead_code)]
        yello: u16,
        #[allow(dead_code)]
        zello: bool,
    },
}

#[test]
fn generate_layout_from_this_file() {
    let container = <RandomStruct as BorshSchema>::schema_container();
    let random_struct_l = Layout::from_borsh_container(container).unwrap();

    assert_eq!(random_struct_l[0].name, "RandomStruct");

    let container = <TestEnum as BorshSchema>::schema_container();
    let test_enum_l = Layout::from_borsh_container(container).unwrap();

    assert_eq!(test_enum_l[0].name, "TestEnum");
    assert_eq!(test_enum_l[1].name, "TestEnumVariantA");
    assert_eq!(test_enum_l[2].name, "TestEnumVariantB");
    assert_eq!(test_enum_l[3].name, "TestEnumVariantC");
    assert_eq!(test_enum_l[4].name, "TestEnumVariantD");
    assert_eq!(test_enum_l[5].name, "TestEnumVariantE");
    assert_eq!(test_enum_l[6].name, "TestEnumVariantF");
    assert_eq!(test_enum_l[7].name, "TestEnumVariantG");

    let mut pubkey_array = [0; 32];
    pubkey_array[31] = 12;
    let enum_variant_a = TestEnum::VariantA;
    let enum_variant_b = TestEnum::VariantB;
    let enum_variant_c = TestEnum::VariantC(1234567890);
    let enum_variant_d = TestEnum::VariantD(Some(Pubkey::new(&pubkey_array)));
    let enum_variant_e = TestEnum::VariantE(None);
    let enum_variant_f = TestEnum::VariantF(RandomStruct {
        field_a: "a test string".to_string(),
        field_b: Some([5, 6, 20, 21]),
    });

    pubkey_array[31] = 22;
    let pubkey_0 = Pubkey::new(&pubkey_array);
    pubkey_array[31] = 23;
    let pubkey_1 = Pubkey::new(&pubkey_array);
    pubkey_array[31] = 24;
    let pubkey_2 = Pubkey::new(&pubkey_array);

    let enum_variant_g = TestEnum::VariantG {
        hello: vec![1, 2, 3, 4, 5],
        bello: [pubkey_0, pubkey_1, pubkey_2],
        yello: 234,
        zello: false,
    };

    let test_data = TestData {
        enum_variant_a: enum_variant_a.try_to_vec().unwrap(),
        enum_variant_b: enum_variant_b.try_to_vec().unwrap(),
        enum_variant_c: enum_variant_c.try_to_vec().unwrap(),
        enum_variant_d: enum_variant_d.try_to_vec().unwrap(),
        enum_variant_e: enum_variant_e.try_to_vec().unwrap(),
        enum_variant_f: enum_variant_f.try_to_vec().unwrap(),
        enum_variant_g: enum_variant_g.try_to_vec().unwrap(),
    };

    fs::create_dir_all(TEST_DATA_DIRECTORY).unwrap();
    let mut file =
        fs::File::create(String::from(TEST_DATA_DIRECTORY) + "/test_enums.json").unwrap();
    write!(file, "{}", serde_json::to_string(&test_data).unwrap()).unwrap();
}


