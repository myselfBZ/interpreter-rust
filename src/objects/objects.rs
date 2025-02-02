use core::fmt;


pub enum Object {
    Int(i32),
    Bool(bool),
    Null
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Object::Int(v) => write!(f, "{v}"),
            Object::Bool(v) => write!(f, "{v}"),
            Object::Null => write!(f, "null")
        }
    }
}


