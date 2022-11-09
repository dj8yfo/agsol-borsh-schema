use super::TEST_DATA_DIRECTORY;
use borsh::{BorshSchema, BorshDeserialize, BorshSerialize};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::pubkey::Pubkey;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use itertools::Itertools;


#[derive(BorshSchema, BorshSerialize, BorshDeserialize)]
pub struct HashMapWrapper {
    map0: HashMap<[u8; 32], Pubkey>,
    map1: HashMap<String, Option<u32>>,
    map2: HashMap<u16, String>,
}

#[test]
fn generate_layout_from_this_file() {
    let mut hashmap0 = HashMap::<[u8; 32], Pubkey>::new();
    let mut id = [0; 32];
    let mut pubkey = [0; 32];
    id[0] = 100;
    pubkey[0] = 10;
    hashmap0.insert(id, Pubkey::new(&pubkey));
    id[1] = 101;
    pubkey[1] = 11;
    hashmap0.insert(id, Pubkey::new(&pubkey));
    id[2] = 102;
    pubkey[2] = 22;
    hashmap0.insert(id, Pubkey::new(&pubkey));
    id[3] = 103;
    pubkey[3] = 33;
    hashmap0.insert(id, Pubkey::new(&pubkey));
    id[4] = 104;
    pubkey[4] = 44;
    hashmap0.insert(id, Pubkey::new(&pubkey));

    let mut hashmap1 = HashMap::<String, Option<u32>>::new();
    hashmap1.insert("hello".to_string(), Some(23));
    hashmap1.insert("bello".to_string(), Some(33));
    hashmap1.insert("yello".to_string(), None);
    hashmap1.insert("zello".to_string(), Some(44));

    let mut hashmap2 = HashMap::<u16, String>::new();
    hashmap2.insert(168, "value".to_string());
    hashmap2.insert(169, "values".to_string());

    let wrapper = HashMapWrapper {
        map0: hashmap0,
        map1: hashmap1,
        map2: hashmap2,
    };

    fs::create_dir_all(TEST_DATA_DIRECTORY).unwrap();
    let mut file =
        fs::File::create(String::from(TEST_DATA_DIRECTORY) + "/test_hashmap.json")
            .unwrap();

    write!(file, "{:?}", wrapper.try_to_vec().unwrap()).unwrap();

    let data = &[
        5, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 101, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 101, 102, 103, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 22,
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 100, 101, 102, 103, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 22, 33, 44, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0,
        98, 101, 108, 108, 111, 1, 33, 0, 0, 0, 5, 0, 0, 0, 104, 101, 108, 108, 111, 1,
        23, 0, 0, 0, 5, 0, 0, 0, 121, 101, 108, 108, 111, 0, 5, 0, 0, 0, 122, 101, 108,
        108, 111, 1, 44, 0, 0, 0, 2, 0, 0, 0, 168, 0, 5, 0, 0, 0, 118, 97, 108, 117,
        101, 169, 0, 6, 0, 0, 0, 118, 97, 108, 117, 101, 115,
    ];

    let wrapper_de = try_from_slice_unchecked::<HashMapWrapper>(data).unwrap();

    for ((key, val), (key_de, val_de)) in wrapper
        .map0
        .iter()
        .sorted()
        .zip(wrapper_de.map0.iter().sorted())
    {
        assert_eq!(key, key_de);
        assert_eq!(val, val_de);
    }

    for ((key, val), (key_de, val_de)) in wrapper
        .map1
        .iter()
        .sorted()
        .zip(wrapper_de.map1.iter().sorted())
    {
        assert_eq!(key, key_de);
        assert_eq!(val, val_de);
    }

    for ((key, val), (key_de, val_de)) in wrapper
        .map2
        .iter()
        .sorted()
        .zip(wrapper_de.map2.iter().sorted())
    {
        assert_eq!(key, key_de);
        assert_eq!(val, val_de);
    }
}


