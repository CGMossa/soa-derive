use syn::{Body, VariantData, MacroInput, Ident, Field};

/// Representing the struct we are deriving
pub struct Struct {
    pub name: Ident,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(input: MacroInput) -> Struct {
        let fields = match input.body {
            Body::Struct(data) => {
                match data {
                    VariantData::Struct(fields) => fields,
                    _ => panic!("#[derive(StructOfArray)] only supports structs."),
                }
            }
            _ => panic!("#[derive(StructOfArray)] only supports structs."),
        };

        Struct {
            name: input.ident,
            fields: fields,
        }
    }
}
