use std::collections::HashSet;
use std::fmt;
use crate::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Eq,Hash, Debug)]
pub enum Axioms {
    SpecialConst,
    A,
    C,
    U
}

#[derive(Clone, PartialEq,Eq, Debug, Hash)]
pub struct FunctionSignature {
    pub name: String,
    pub arity: usize,
    pub axioms: Vec<Axioms>,
}
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Function {
    pub signature: FunctionSignature,
    pub args: Vec<Term>,
}

pub type Signature = Vec<FunctionSignature>;


impl FunctionSignature {
    pub fn new(name: String, arity: usize,axioms: Vec<Axioms>) -> FunctionSignature {
        FunctionSignature{name, arity,axioms}
    }
}

impl Function {
    pub fn new(signature: &FunctionSignature, args: &Vec<Term>) -> Function {
        Function{signature: signature.clone(), args: args.clone()}
    }
}


impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.signature.arity == 0 {
            write!(f, "{}", self.signature.name)
        }
        else {
            let args_str = self.args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ");

            write!(f, "{}({})", self.signature.name, args_str)
        }
    }
}
