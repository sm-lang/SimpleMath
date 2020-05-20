use crate::AST;
use crate::evaluate::utils::CheckAttributes;

impl CheckAttributes for AST {
    fn is_string(&self) -> bool {
        match &self {
            AST::String(..) => true,
            _ => false
        }
    }
    fn is_boolean(&self) -> bool {
        match &self {
            AST::Boolean(..) => true,
            _ => false
        }
    }
    fn is_null(&self) -> bool {
        match &self {
            AST::Null => true,
            _ => false
        }
    }
}