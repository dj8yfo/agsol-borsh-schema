use super::BorshType;
use heck::MixedCase;

use std::str::FromStr;
use anyhow::anyhow;

/// Represents a field in a TypeScript class and a borsh schema.
#[derive(Debug)]
pub struct LayoutField {
    name: String,
    ty: BorshType,
}

impl LayoutField {
    pub fn from_declaration(
        name: Option<&str>,
        field: &str,
        idx: Option<usize>,
    ) -> Result<Self, anyhow::Error> {
        let name = if let Some(name) = name {
            name.to_mixed_case()
        } else {
            format!(
                "unnamed_{}",
                idx.map_or(Err(anyhow!("no index provided")), Ok)?
            )
        };
        let ty = BorshType::from_str(field)?;
        Ok(Self { name, ty })
    }

    pub fn from_enum_variant(name_str: &str) -> Result<Self, anyhow::Error> {
        let ty = BorshType::from_str(name_str)?;
        Ok(Self {
            name: name_str.to_mixed_case(),
            ty,
        })
    }

    /// Converts the field into a TypeScript class field representation.
    pub fn to_class_field(&self) -> String {
        format!("{}: {}", self.name, self.ty.to_class_type())
    }

    /// Converts the field into a borsh schema field representation.
    pub fn to_borsh_schema(&self) -> String {
        format!("['{}', {}]", self.name, self.ty.to_borsh_schema())
    }

    pub fn should_skip(&self) -> bool {
        self.ty == BorshType::Skip
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_field_construction() {
        let field =
            LayoutField::from_declaration(Some(&"random_field".to_owned()), "u8", None)
                .unwrap();

        assert_eq!(field.name, "randomField");
        assert_eq!(field.ty, BorshType::U8);
    }

    #[test]
    fn complex_field_construction() {
        let field = LayoutField::from_declaration(
            Some(&"optional_accounts".to_owned()),
            "Array<Option<Pubkey>, 3>",
            None,
        )
        .unwrap();

        assert_eq!(field.name, "optionalAccounts");
        assert_eq!(
            field.ty,
            BorshType::FixedArray(
                Box::new(BorshType::Option(Box::new(BorshType::Pubkey))),
                3
            )
        );
    }

    #[test]
    fn simple_field_to_borsh_schema() {
        let field = LayoutField {
            name: "someRandomString".to_owned(),
            ty: BorshType::String,
        };

        assert_eq!(field.to_borsh_schema(), "['someRandomString', 'string']");

        let field = LayoutField {
            name: "myCustomType".to_owned(),
            ty: BorshType::Custom("aCustomType".to_owned()),
        };

        assert_eq!(field.to_borsh_schema(), "['myCustomType', aCustomType]");
    }

    #[test]
    fn field_to_ts_class_field() {
        let field = LayoutField {
            name: "fieldAlpha".to_owned(),
            ty: BorshType::U64,
        };
        assert_eq!(field.to_class_field(), "fieldAlpha: BN");
        let field = LayoutField {
            name: "fieldBeta".to_owned(),
            ty: BorshType::Vec(Box::new(BorshType::String)),
        };
        assert_eq!(field.to_class_field(), "fieldBeta: string[]");
        let field = LayoutField {
            name: "fieldGamma".to_owned(),
            ty: BorshType::Option(Box::new(BorshType::FixedBytes(32))),
        };
        assert_eq!(field.to_class_field(), "fieldGamma: Uint8Array | null");
    }
}
