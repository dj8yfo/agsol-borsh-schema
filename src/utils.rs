//! Utility functions that perform file parsing and layout generation before
//! writing the result into a schema file.
//!
//! # Example
//!
//! ```rust
//! # use agsol_borsh_schema::{generate_layouts, generate_output};
//! # fn generate_schema() -> Result<(), anyhow::Error> {
//!     let layouts = generate_layouts("~/input-rust-directory")?;
//!     generate_output(&layouts, "./output-directory")?;
//! # Ok(())
//! # }
//!
//! ```

use crate::layout::{Kind, Layout, LayoutField};

use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;


/// Traverses all files in the input directory and looks for data structures
/// that should be converted into TypeScript and borsh layouts.
pub fn generate_layouts(directory: impl AsRef<Path>) -> Result<Vec<Layout>, anyhow::Error> {
    let dir_items = fs::read_dir(directory)?;
    Ok(dir_items
        .flat_map(|item| {
            let path = item?.path();
            if path.is_dir() {
                generate_layouts(path)
            } else {
                generate_layout_from_file(path)
            }
        })
        .flatten()
        .collect::<Vec<Layout>>())
}

/// Generates the TypeScript and borsh layouts of rust data structures found in `.rs`
/// files.
pub fn generate_layout_from_file(filepath: impl AsRef<Path>) -> Result<Vec<Layout>, anyhow::Error> {
    if filepath.as_ref().extension() != Some(OsStr::new("rs")) {
        return Ok(Vec::new());
    }
    let code = fs::read_to_string(filepath)?;
    let syntax = syn::parse_file(&code)?;
    let mut layouts = Vec::<Layout>::new();
    for item in syntax.items {
        match item {
            syn::Item::Struct(ref item_struct) => {
                for attr in &item_struct.attrs {
                    let attribute_string = attr.tokens.to_string();
                    if attribute_string.contains(crate::ATTRIBUTE_LABEL) {
                        layouts.push(Layout::from_tokens(
                            &item_struct.ident.to_string(),
                            &mut item_struct.fields.iter(),
                        )?);
                    }
                }
            }
            syn::Item::Enum(ref item_enum) => {
                for attr in &item_enum.attrs {
                    let attribute_string = attr.tokens.to_string();
                    if attribute_string.contains(crate::ATTRIBUTE_LABEL) {
                        let mut enum_layout = Layout {
                            name: item_enum.ident.to_string(),
                            kind: Kind::Enum,
                            fields: Vec::new(),
                        };
                        let parent_name = item_enum.ident.to_string();
                        let mut variant_layouts = item_enum
                            .variants
                            .iter()
                            .map(|variant| {
                                let name = parent_name.clone() + &variant.ident.to_string();
                                enum_layout
                                    .fields
                                    .push(LayoutField::from_enum_variant(&name)?);
                                Layout::from_tokens(&name, &mut variant.fields.iter())
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        layouts.push(enum_layout);
                        layouts.append(&mut variant_layouts);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(layouts)
}

// TODO: move borshPublicKey to 'ts-borsh-schema'
static LIB_PREABMLE: &str = r#"import { Struct, Enum } from 'ts-borsh-schema';
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { borshPublicKey } from "./extensions/publicKey";

borshPublicKey();

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
