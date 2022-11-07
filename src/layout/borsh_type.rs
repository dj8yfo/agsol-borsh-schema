use std::str::FromStr;

/// Types that can be represented in a TypsScript borsh schema
/// for (de)serialization.
#[derive(Clone, Debug, PartialEq)]
pub enum BorshType {
    U8,
    U16,
    U32,
    U64,
    U128,
    Bool,
    String,
    Pubkey,
    Vec(Box<BorshType>),
    FixedArray(Box<BorshType>, usize),
    FixedBytes(usize),
    Option(Box<BorshType>),
    Map(Box<BorshType>, Box<BorshType>),
    Custom(String),
    Skip,
}
use anyhow::anyhow;
use quote::ToTokens;
use syn::Type;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn proc_option(x: String) -> Result<BorshType, anyhow::Error> {
    let inner = x
        .strip_prefix("Option<")
        .unwrap()
        .strip_suffix('>')
        .ok_or_else(|| anyhow::anyhow!("invalid Option"))?;
    let inner_type = BorshType::from_str(inner)?;
    Ok(BorshType::Option(Box::new(inner_type)))
}

fn proc_vec(x: String) -> Result<BorshType, anyhow::Error> {
    let inner = x
        .strip_prefix("Vec<")
        .unwrap()
        .strip_suffix('>')
        .ok_or_else(|| anyhow::anyhow!("invalid Vec"))?;
    let inner_type = BorshType::from_str(inner)?;
    Ok(BorshType::Vec(Box::new(inner_type)))
}

fn proc_vec_deque(x: String) -> Result<BorshType, anyhow::Error> {
    let inner = x
        .strip_prefix("VecDeque<")
        .unwrap()
        .strip_suffix('>')
        .ok_or_else(|| anyhow::anyhow!("invalid VecDeque"))?;
    let inner_type = BorshType::from_str(inner)?;
    Ok(BorshType::Vec(Box::new(inner_type)))
}

fn proc_arr_old(x: String) -> Result<BorshType, anyhow::Error> {
    let inner = x
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .ok_or_else(|| anyhow::anyhow!("invalid array, missing ']'"))?;
    let (array_type_str, array_len_str) = inner
        .rsplit_once(';')
        .ok_or_else(|| anyhow::anyhow!("invalid array, missing ';'"))?;
    let array_type = BorshType::from_str(array_type_str)?;
    let array_len = array_len_str.parse::<usize>()?;
    if let BorshType::U8 = array_type {
        Ok(BorshType::FixedBytes(array_len))
    } else {
        Ok(BorshType::FixedArray(Box::new(array_type), array_len))
    }
}

fn proc_arr(x: String) -> Result<BorshType, anyhow::Error> {
    let inner = x
        .strip_prefix("Array<")
        .unwrap()
        .strip_suffix('>')
        .ok_or_else(|| anyhow::anyhow!("invalid array, missing '>'"))?;
    let (array_type_str, array_len_str) = inner
        .rsplit_once(',')
        .ok_or_else(|| anyhow::anyhow!("invalid array, missing ','"))?;
    let array_type = BorshType::from_str(array_type_str)?;
    let array_len = array_len_str.parse::<usize>()?;
    if let BorshType::U8 = array_type {
        Ok(BorshType::FixedBytes(array_len))
    } else {
        Ok(BorshType::FixedArray(Box::new(array_type), array_len))
    }
}

fn split_hash_map(input: &str) -> Result<(String, String), anyhow::Error> {
    let syntax: Type = syn::parse_str(input)?;
    match syntax {
        Type::Path(type_path) => {
            let result = match &type_path.path.segments[0].arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    let l_arg = &args.args[0];
                    let r_arg = &args.args[1];

                    let mut l_str = l_arg.to_token_stream().to_string();
                    let mut r_str = r_arg.to_token_stream().to_string();
                    remove_whitespace(&mut l_str);
                    remove_whitespace(&mut r_str);

                    Ok((l_str, r_str))
                }
                _ => Err(anyhow!("wrong variant")),
            };

            // TODO: remove debug output
            println!("{:?}", result);
            result
        }
        _ => Err(anyhow!("wrong variant")),
    }
}

fn proc_hash_map(input: String) -> Result<BorshType, anyhow::Error> {
    let (key_str, value_str) = split_hash_map(&input)?;
    let key = BorshType::from_str(&key_str)?;
    let value = BorshType::from_str(&value_str)?;
    Ok(BorshType::Map(Box::new(key), Box::new(value)))
}
fn if_starts_with_patterns(input: String) -> Result<BorshType, anyhow::Error> {
    match input {
        x if x.strip_prefix("Option<").is_some() => proc_option(x),
        x if x.strip_prefix("Vec<").is_some() => proc_vec(x),
        x if x.strip_prefix("VecDeque<").is_some() => proc_vec_deque(x),
        x if x.strip_prefix('[').is_some() => proc_arr_old(x),
        x if x.strip_prefix("Array<").is_some() => proc_arr(x),
        x if x.strip_prefix("HashMap<").is_some() => proc_hash_map(x),
        _ => Ok(BorshType::Custom(input)),
    }
}

impl FromStr for BorshType {
    type Err = anyhow::Error;
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let mut input = input_str.to_owned();
        input.retain(|c| !c.is_whitespace());
        match input.as_ref() {
            "u8" | "i8" => Ok(BorshType::U8),
            "u16" | "i16" => Ok(BorshType::U16),
            "u32" | "i32" => Ok(BorshType::U32),
            "u64" | "i64" | "UnixTimestamp" => Ok(BorshType::U64),
            "u128" | "i128" => Ok(BorshType::U128),
            "bool" => Ok(BorshType::Bool),
            "String" | "string" => Ok(BorshType::String),
            "Pubkey" => Ok(BorshType::Pubkey),
            _ => if_starts_with_patterns(input),
        }
    }
}

impl BorshType {
    /// Converts the type to a schema representation used by
    /// [`borsh-js`](https://github.com/near/borsh-js).
    pub fn to_borsh_schema(&self) -> String {
        match self {
            Self::U8 | Self::Bool => "'u8'".to_owned(),
            Self::U16 => "'u16'".to_owned(),
            Self::U32 => "'u32'".to_owned(),
            Self::U64 => "'u64'".to_owned(),
            Self::U128 => "'u128'".to_owned(),
            Self::String => "'string'".to_owned(),
            Self::Pubkey => "'publicKeyHack'".to_owned(),
            Self::Vec(inner) => format!("[{}]", inner.to_borsh_schema()),
            Self::FixedArray(inner, len) => {
                format!("[{}, {}]", inner.to_borsh_schema(), len)
            }
            Self::FixedBytes(len) => format!("[{}]", len),
            Self::Option(inner) => {
                format!("{{ kind: 'option', type: {} }}", inner.to_borsh_schema())
            }
            Self::Map(key, value) => format!(
                "{{ kind: 'map', key: {}, value: {} }}",
                key.to_borsh_schema(),
                value.to_borsh_schema()
            ),
            Self::Custom(inner) => inner.to_owned(),
            _ => unimplemented!(),
        }
    }

    /// Converts the type to a TypeScript class type.
    pub fn to_class_type(&self) -> String {
        match self {
            Self::U8 => "number".to_owned(),
            Self::U16 => "number".to_owned(),
            Self::U32 => "number".to_owned(),
            Self::U64 => "BN".to_owned(),
            Self::U128 => "BN".to_owned(),
            Self::Bool => "boolean".to_owned(),
            Self::String => "string".to_owned(),
            Self::Pubkey => "PublicKey".to_owned(),
            Self::Vec(inner) => format!("{}[]", inner.to_class_type()),
            Self::FixedArray(inner, _len) => format!("{}[]", inner.to_class_type()),
            Self::FixedBytes(len) => format!("[{}]", len),
            Self::Option(inner) => {
                format!("{} | null", inner.to_class_type())
            }
            Self::Map(key, value) => {
                format!("Map<{}, {}>", key.to_class_type(), value.to_class_type())
            }
            Self::Custom(inner) => inner.to_owned(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_type_from_str() {
        assert_eq!(BorshType::from_str("bool").unwrap(), BorshType::Bool);
        assert_eq!(BorshType::from_str("u8").unwrap(), BorshType::U8);
        assert_eq!(BorshType::from_str("i8").unwrap(), BorshType::U8);
        assert_eq!(BorshType::from_str("u16").unwrap(), BorshType::U16);
        assert_eq!(BorshType::from_str("i16").unwrap(), BorshType::U16);
        assert_eq!(BorshType::from_str("u32").unwrap(), BorshType::U32);
        assert_eq!(BorshType::from_str("i32").unwrap(), BorshType::U32);
        assert_eq!(BorshType::from_str("u64").unwrap(), BorshType::U64);
        assert_eq!(BorshType::from_str("i64").unwrap(), BorshType::U64);
        assert_eq!(BorshType::from_str("u128").unwrap(), BorshType::U128);
        assert_eq!(BorshType::from_str("i128").unwrap(), BorshType::U128);
        assert_eq!(BorshType::from_str("String").unwrap(), BorshType::String);
        assert_eq!(BorshType::from_str("Pubkey").unwrap(), BorshType::Pubkey);
        assert_eq!(
            BorshType::from_str("SomeTestStruct").unwrap(),
            BorshType::Custom("SomeTestStruct".to_owned())
        );
    }

    #[test]
    fn complex_type_from_str() {
        assert_eq!(
            BorshType::from_str("Vec<u8>").unwrap(),
            BorshType::Vec(Box::new(BorshType::U8))
        );
        assert_eq!(
            BorshType::from_str("VecDeque<u64>").unwrap(),
            BorshType::Vec(Box::new(BorshType::U64))
        );
        assert_eq!(
            BorshType::from_str("Option<u64>").unwrap(),
            BorshType::Option(Box::new(BorshType::U64))
        );
        assert_eq!(
            BorshType::from_str("Vec<Option<[Pubkey; 2]>>").unwrap(),
            BorshType::Vec(Box::new(BorshType::Option(Box::new(
                BorshType::FixedArray(Box::new(BorshType::Pubkey), 2)
            ))))
        );
        assert_eq!(
            BorshType::from_str("[[Option<i32>; 2]; 4]").unwrap(),
            BorshType::FixedArray(
                Box::new(BorshType::FixedArray(
                    Box::new(BorshType::Option(Box::new(BorshType::U32))),
                    2
                )),
                4
            )
        );

        assert_eq!(
            BorshType::from_str("Option<Vec<OtherStruct>>").unwrap(),
            BorshType::Option(Box::new(BorshType::Vec(Box::new(BorshType::Custom(
                "OtherStruct".to_owned()
            )))))
        );

        assert_eq!(
            BorshType::from_str("[u8; 32]").unwrap(),
            BorshType::FixedBytes(32),
        );
    }

    #[test]
    fn simple_type_to_borsh() {
        assert_eq!(BorshType::Bool.to_borsh_schema(), "'u8'");
        assert_eq!(BorshType::U8.to_borsh_schema(), "'u8'");
        assert_eq!(BorshType::U16.to_borsh_schema(), "'u16'");
        assert_eq!(BorshType::U32.to_borsh_schema(), "'u32'");
        assert_eq!(BorshType::U64.to_borsh_schema(), "'u64'");
        assert_eq!(BorshType::U128.to_borsh_schema(), "'u128'");
        assert_eq!(BorshType::String.to_borsh_schema(), "'string'");
        assert_eq!(BorshType::Pubkey.to_borsh_schema(), "'publicKeyHack'");
    }

    #[test]
    fn complex_type_to_borsh() {
        assert_eq!(BorshType::FixedBytes(32).to_borsh_schema(), "[32]",);
        assert_eq!(
            BorshType::Vec(Box::new(BorshType::U64)).to_borsh_schema(),
            "['u64']"
        );
        assert_eq!(
            BorshType::from_str("Option<Vec<OtherStruct>>")
                .unwrap()
                .to_borsh_schema(),
            "{ kind: 'option', type: [OtherStruct] }"
        );

        assert_eq!(
            BorshType::from_str("[[Option<i32>; 2]; 4]")
                .unwrap()
                .to_borsh_schema(),
            "[[{ kind: 'option', type: 'u32' }, 2], 4]"
        );

        assert_eq!(
            BorshType::from_str("HashMap<[u8; 32], Pubkey>")
                .unwrap()
                .to_borsh_schema(),
            "{ kind: 'map', key: [32], value: 'publicKeyHack' }"
        );
        assert_eq!(
            BorshType::from_str("HashMap<Array<u8, 32>, Pubkey>")
                .unwrap()
                .to_borsh_schema(),
            "{ kind: 'map', key: [32], value: 'publicKeyHack' }"
        );
        assert_eq!(
            BorshType::from_str("HashMap<string, Option<u32>>")
                .unwrap()
                .to_borsh_schema(),
            "{ kind: 'map', key: 'string', value: { kind: 'option', type: 'u32' } }"
        );
    }

    #[test]
    fn types_to_ts() {
        let ty = BorshType::from_str("u64").unwrap();
        assert_eq!(ty.to_class_type(), "BN");
        let ty = BorshType::from_str("Option<Vec<Pubkey>>").unwrap();
        assert_eq!(ty.to_class_type(), "PublicKey[] | null");
        let ty = BorshType::from_str("[bool; 5]").unwrap();
        assert_eq!(ty.to_class_type(), "boolean[]");
        let ty = BorshType::from_str("HashMap<[u8; 32], PublicKey>").unwrap();
        assert_eq!(dbg!(ty.to_class_type()), "Map<[32], PublicKey>");
    }
}
