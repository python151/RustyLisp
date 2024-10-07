use std::fmt;


pub struct FunctionStruct {
    pub name: String,
    pub function: fn(Vec<Symbol>) -> Symbol,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Str(String),
    Int(i32),
    Double(f64),
    Function(FunctionStruct),
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
            Symbol::Function(fun) => write!(f, "Function: {}", fun.name),
        }
    }
}

impl Clone for FunctionStruct {
    fn clone(&self) -> Self {
        FunctionStruct {
            name: self.name.clone(),
            function: self.function.clone()
        }
    }
}

impl fmt::Debug for FunctionStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FunctionStruct {{ name: {} }}", self.name)
    }
}
