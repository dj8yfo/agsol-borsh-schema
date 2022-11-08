//! Utility functions that perform file parsing and layout generation before
//! writing the result into a schema file.

use crate::layout::Layout;

use std::fs;
use std::io::Write;
use std::path::Path;


// TODO: move borshPublicKeyHack to 'ts-borsh-schema'
static LIB_PREABMLE: &str = r#"import { Struct, Enum } from 'ts-borsh-schema';
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { borshPublicKeyHack } from "./extensions/publicKey";

borshPublicKeyHack();

"#;
/// Writes the generated layouts into a file in the provided output directory.
pub fn generate_output(
    layouts: &[Layout],
    output_directory: impl AsRef<Path>,
) -> Result<(), anyhow::Error> {
    let schema_string = layouts
        .iter()
        .map(|layout| layout.to_borsh_schema())
        .collect::<String>();

    let classes_string = layouts
        .iter()
        .map(|layout| layout.to_ts_class())
        .collect::<String>();


    let schema = format!(
        r#"export const SCHEMA = new Map<any, any>([{}
]);"#,
        schema_string
    );

    let imports = String::from(LIB_PREABMLE);

    fs::create_dir_all(&output_directory)?;
    let mut file = fs::File::create(output_directory.as_ref().join("schema.ts"))?;
    write!(file, "{}", imports + &classes_string + &schema)?;
    Ok(())
}
