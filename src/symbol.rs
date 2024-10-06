use std::fmt;

#[derive(Debug, Clone)]
pub enum Symbol {
    Str(String),
    Int(i64),
    Double(f64),
    Sexp,
    None
} 

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Symbol::None => write!(f, "None"),
            &Symbol::Sexp => write!(f, "Sexp"),
            Symbol::Str(s) => write!(f, "String value: {}", s),
            Symbol::Int(i) => write!(f, "Integer value: {}", i),
            Symbol::Double(s) => write!(f, "Double value: {}", s),
        }
    }
}