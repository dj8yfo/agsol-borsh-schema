
use super::Kind;
use super::Layout;
// use super::Layout;
use super::LayoutField;
use borsh::schema::BorshSchemaContainer;
use borsh::schema::Declaration;
use borsh::schema::Fields;
use borsh::schema::VariantName;


fn match_fields(fields: &Fields) -> Result<Vec<LayoutField>, anyhow::Error> {
    #[cfg(test)]
    dbg!(fields);
    type FuncResult = Result<Vec<LayoutField>, anyhow::Error>;
    let result = match fields {
        Fields::NamedFields(names_types) => {
            let result: FuncResult =names_types.iter().map(|(name, field)|{
                LayoutField::from_declaration(Some(name), field, None)
            }).collect(); 
            result
        }
        Fields::UnnamedFields(types) => {
            let result: FuncResult = types.iter().enumerate().map(|(idx, field)| {
                LayoutField::from_declaration(None, field, Some(idx))
            }).collect();
            result
        }
        Fields::Empty => {Ok(vec![])}
    };
    result
}
type EnumResult = (Vec<LayoutField>, Vec<Layout>);

fn match_enum_variants(
    variants: &[(VariantName, Declaration)],
    container: &BorshSchemaContainer,
) -> Result<EnumResult, anyhow::Error> {
    #[cfg(test)]
    dbg!(variants);
    let (fields, layouts): (Vec<_>, Vec<_>) = variants
        .iter()
        .map(|(_variant_name, declaration)| {
            let layout_field = LayoutField::from_enum_variant(declaration);
            let layout = Layout::from_borsh_definition(declaration, container)
                // we assume an enum variant definition being just a single Kind::Struct
                // layout
                .map(|mut vec| vec.remove(0));
            (layout_field, layout)
        })
        .unzip();
    let fields: Result<Vec<LayoutField>, anyhow::Error> = fields.into_iter().collect();
    let layouts: Result<Vec<Layout>, anyhow::Error> = layouts.into_iter().collect();

    Ok((fields?, layouts?))
}

impl Layout {
    fn from_borsh_definition(
        declaration: &borsh::schema::Declaration, 
        container: &BorshSchemaContainer,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let definition = &container.definitions[declaration];
        let kind: Kind;

        #[cfg(test)]
        dbg!(declaration, definition);

        let mut result = vec![];
        let (fields, nested_layouts): (Vec<LayoutField>, Vec<Self>) = match definition {
            borsh::schema::Definition::Struct { fields } => {
                kind = Kind::Struct;
                (match_fields(fields)?, vec![])
            },
            borsh::schema::Definition::Enum { variants }  => { 
                kind = Kind::Enum;
                match_enum_variants(variants, container)?
            },
            _ => unimplemented!("unexpected variant for ts borsh schema generation"),
        };
        result.push(Self {
            name: declaration.clone(),
            kind,
            fields,
        });
        result.extend(nested_layouts);

        Ok(result)
    }
    /// Generates a layout from the underlying token stream.
    pub fn from_borsh_container(
        container: BorshSchemaContainer,
    ) -> Result<Vec<Self>, anyhow::Error> {
        Self::from_borsh_definition(&container.declaration, &container)
    }
}
