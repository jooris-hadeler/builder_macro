use syn::Ident;

/// Represents the builder struct.
pub struct Builder {
    /// The name of the struct being built.
    pub name: Ident,
    /// The fields of the builder struct.
    pub fields: Vec<BuilderField>,
}

impl Builder {
    /// Creates a new builder.
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    /// Adds a field to the builder.
    pub fn add_field(&mut self, field: BuilderField) {
        self.fields.push(field);
    }
}

/// Represents a field in the builder struct.
pub struct BuilderField {
    /// The name of the field.
    pub name: Ident,
    /// The type of the field.
    pub ty: syn::Type,
    /// The default value of the field.
    pub default: Option<syn::Expr>,
    /// Whether the field should be skipped.
    pub skip: bool,
}

impl BuilderField {
    /// Creates a new builder field.
    pub fn new(name: Ident, ty: syn::Type, default: Option<syn::Expr>, skip: bool) -> Self {
        Self {
            name,
            ty,
            default,
            skip,
        }
    }
}
