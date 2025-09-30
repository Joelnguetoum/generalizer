use std::fmt;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature, Signature};

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Term {
    Function(Function),
    Variable(Variable),
}



impl Term {

    pub fn is_special_constant(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.args.len()!=0{
                    return false;
                }

                if f.signature.arity==0 && f.signature.axioms.contains(&Axioms::SpecialConst){
                    return true;
                }

                false
            },
            Term::Variable(v)=>false
        }
    }

    pub fn apply_substitution(&self, sub: &Substitution)-> Term {
        match self.clone() {
            Term::Variable(var) => {
                if let Some(t) = sub.map.get(&var){
                    t.clone()
                }else{
                    self.clone()
                }
            },
            Term::Function(f) => {
                let mut args = Vec::new();

                for arg in &f.args {
                    args.push(arg.apply_substitution(sub));
                }

                Term::Function(Function::new(&f.signature,&args))
            }
        }
    }

    pub fn head(&self) -> String {
        match self.clone() {
            Term::Variable(v)=> v.name(),
            Term::Function(f)=> f.signature.name
        }
    }

    pub fn head_ground(&self) -> FunctionSignature {
        match self.clone() {
            Term::Variable(v)=> panic!("Applied head_ground to a non-gound term"),
            Term::Function(f)=> f.signature
        }
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Function(func) => write!(f, "{}", func),
            Term::Variable(v) => write!(f, "{}", v)
        }
    }
}