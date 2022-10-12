use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Any,
    Number,
    String,
    Boolean,
    Table,
    Function,
}

impl FromStr for Types {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "any" => Ok(Self::Any),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "boolean" => Ok(Self::Boolean),
            "table" => Ok(Self::Table),
            "function" => Ok(Self::Function),
            "()->()" => Ok(Self::Function),
            "() -> ()" => Ok(Self::Function),
            _ => Err(()),
        }
    }
}
