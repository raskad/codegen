use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
    annotation: Vec<String>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
            annotation: Vec::new(),
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Add an annotation to a variant.
    pub fn annotation(&mut self, annotation: Vec<&str>) -> &mut Self {
        self.annotation = annotation.iter().map(|ann| ann.to_string()).collect();
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.annotation.is_empty() {
            for ann in &self.annotation {
                write!(fmt, "{}\n", ann)?;
            }
        }

        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
