
use super::Kind;
// use super::Layout;
use super::LayoutField;
use borsh::schema::BorshSchemaContainer;
use borsh::schema::Fields;


fn match_fields(fields: &Fields) -> Result<Vec<LayoutField>, anyhow::Error> {
    #[cfg(test)]
    dbg!(fields);
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
    fn from_borsh_definition(
        declaration: &borsh::schema::Declaration, 
        container: &BorshSchemaContainer,
    ) -> Result<Self, anyhow::Error> {
        let definition = &container.definitions[declaration];
        let mut kind: Kind = Kind::Struct;

        #[cfg(test)]
        dbg!(declaration, definition);
        #[allow(clippy::single_match)]
        let fields = match definition {
            borsh::schema::Definition::Struct { fields } => {
                kind = Kind::Struct;
                match_fields(fields)?
            },
            _ => { vec![]},
        };

        Ok(Self {
            name: declaration.clone(),
            kind,
            fields,
        })
    }
    /// Generates a layout from the underlying token stream.
    pub fn from_borsh_container(
        container: BorshSchemaContainer,
    ) -> Result<Self, anyhow::Error> {
        Self::from_borsh_definition(&container.declaration, &container)
    }
}
