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
    Bool(bool),
    Sexp,
} 

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Symbol::Sexp => write!(f, "Sexp"),
            Symbol::Str(s) => write!(f, "String value: {}", s),
            Symbol::Bool(s) => write!(f, "Bool value: {}", s),
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

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Symbol::Str(str) => {
                if let Symbol::Str(str1) = other {
                    return *str == *str1;
                }
                false
            }
            Symbol::Bool(b) => {
                if let Symbol::Bool(b1) = other {
                    return *b == *b1;
                }
                false
            }
            Symbol::Int(int) => {
                if let Symbol::Int(int1) = other {
                    return *int == *int1;
                }
                false
            }
            Symbol::Double(doub) => {
                if let Symbol::Double(doub1) = other {
                    return ((*doub)*(*doub) - (*doub1)*(*doub1)) < 0.0000001
                            || (((*doub1)*(*doub1) - (*doub)*(*doub)) < 0.0000001);
                }
                false
            }
            Symbol::Function(fun) => {
                if let Symbol::Function(fun1) = other {
                    return fun.name == fun1.name;
                }
                false
            }
            Symbol::Sexp => {
                if matches!(other, Symbol::Sexp) {
                    return true;
                }
                false
            }
        }
    }
}