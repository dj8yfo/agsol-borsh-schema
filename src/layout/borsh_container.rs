
use super::Kind;
use super::Layout;
use super::LayoutField;
use borsh::schema::BorshSchemaContainer;
use borsh::schema::Fields;
use quote::ToTokens;
use syn::Type;


fn match_fields(fields: &Fields) -> Result<Vec<LayoutField>, anyhow::Error> {
    println!("{:#?}", fields);
    let vec = match fields {
        Fields::NamedFields(names_types) => {
            for (name, elem) in names_types {
            }
            vec![]
        }
        Fields::UnnamedFields(types) => {vec![]}
        Fields::Empty => {vec![]}
    };
    Ok(vec)
}

impl super::Layout {
    /// Generates a layout from the underlying token stream.
    pub fn from_borsh_container(
        container: BorshSchemaContainer,
    ) -> Result<Self, anyhow::Error> {

        let definition = &container.definitions[&container.declaration];
        let mut kind: Kind = Kind::Struct;
        println!("{:#?}", definition);
        match definition {
            borsh::schema::Definition::Struct { fields } => {
                kind = Kind::Struct;
                match_fields(fields);
            },
            _ => {},
        };

        let fields = Vec::new();
        Ok(Self {
            name: container.declaration,
            kind,
            fields,
        })
    }
}
