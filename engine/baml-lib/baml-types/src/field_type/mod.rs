use crate::BamlMediaType;
use crate::Constraint;

mod builder;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum TypeValue {
    String,
    Int,
    Float,
    Bool,
    // Char,
    Null,
    Media(BamlMediaType),
}

impl std::str::FromStr for TypeValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "string" => TypeValue::String,
            "int" => TypeValue::Int,
            "float" => TypeValue::Float,
            "bool" => TypeValue::Bool,
            "null" => TypeValue::Null,
            "image" => TypeValue::Media(BamlMediaType::Image),
            "audio" => TypeValue::Media(BamlMediaType::Audio),
            _ => return Err(()),
        })
    }
}

impl std::fmt::Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::String => write!(f, "string"),
            TypeValue::Int => write!(f, "int"),
            TypeValue::Float => write!(f, "float"),
            TypeValue::Bool => write!(f, "bool"),
            TypeValue::Null => write!(f, "null"),
            TypeValue::Media(BamlMediaType::Image) => write!(f, "image"),
            TypeValue::Media(BamlMediaType::Audio) => write!(f, "audio"),
        }
    }
}

/// Subset of [`crate::BamlValue`] allowed for literal type definitions.
#[derive(serde::Serialize, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum LiteralValue {
    String(String),
    Int(i64),
    Bool(bool),
}

impl LiteralValue {
    pub fn literal_base_type(&self) -> FieldType {
        match self {
            Self::String(_) => FieldType::string(),
            Self::Int(_) => FieldType::int(),
            Self::Bool(_) => FieldType::bool(),
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::String(str) => write!(f, "\"{str}\""),
            LiteralValue::Int(int) => write!(f, "{int}"),
            LiteralValue::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

/// FieldType represents the type of either a class field or a function arg.
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum FieldType {
    Primitive(TypeValue),
    Enum(String),
    Literal(LiteralValue),
    Class(String),
    List(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
    Union(Vec<FieldType>),
    Tuple(Vec<FieldType>),
    Optional(Box<FieldType>),
    RecursiveTypeAlias(String),
    Constrained {
        base: Box<FieldType>,
        constraints: Vec<Constraint>,
    },
}

// Impl display for FieldType
impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Enum(name)
            | FieldType::Class(name)
            | FieldType::RecursiveTypeAlias(name) => write!(f, "{name}"),
            FieldType::Primitive(t) => write!(f, "{t}"),
            FieldType::Literal(v) => write!(f, "{v}"),
            FieldType::Union(choices) => {
                write!(
                    f,
                    "({})",
                    choices
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            FieldType::Tuple(choices) => {
                write!(
                    f,
                    "({})",
                    choices
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            FieldType::Map(k, v) => write!(f, "map<{k}, {v}>"),
            FieldType::List(t) => write!(f, "{t}[]"),
            FieldType::Optional(t) => write!(f, "{t}?"),
            FieldType::Constrained { base, .. } => base.fmt(f),
        }
    }
}

impl FieldType {
    pub fn is_primitive(&self) -> bool {
        match self {
            FieldType::Primitive(_) => true,
            FieldType::Optional(t) => t.is_primitive(),
            FieldType::List(t) => t.is_primitive(),
            FieldType::Constrained { base, .. } => base.is_primitive(),
            _ => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            FieldType::Optional(_) => true,
            FieldType::Primitive(TypeValue::Null) => true,
            FieldType::Union(types) => types.iter().any(FieldType::is_optional),
            FieldType::Constrained { base, .. } => base.is_optional(),
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            FieldType::Primitive(TypeValue::Null) => true,
            FieldType::Optional(t) => t.is_null(),
            FieldType::Constrained { base, .. } => base.is_null(),
            _ => false,
        }
    }
}
