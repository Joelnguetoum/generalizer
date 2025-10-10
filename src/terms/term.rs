use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature, Signature};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
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

    pub fn is_alpha_equivalent(&self, other: &Term) -> bool {
        fn helper(t1: &Term, t2: &Term, map1: &mut HashMap<String, String>, map2: &mut HashMap<String, String>) -> bool {
            match (t1, t2) {
                (Term::Variable(v1), Term::Variable(v2)) => {
                    let n1 = &v1.name();
                    let n2 = &v2.name();
                    match (map1.get(n1), map2.get(n2)) {
                        (Some(m1), Some(m2)) => m1 == n2 && m2 == n1,
                        (None, None) => {
                            map1.insert(n1.clone(), n2.clone());
                            map2.insert(n2.clone(), n1.clone());
                            true
                        }
                        _ => false,
                    }
                }
                (Term::Function(f1), Term::Function(f2)) => {
                    if f1.signature != f2.signature || f1.args.len() != f2.args.len() {
                        return false;
                    }
                    f1.args
                        .iter()
                        .zip(&f2.args)
                        .all(|(a, b)| helper(a, b, map1, map2))
                }
                _ => false,
            }
        }

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        helper(self, other, &mut map1, &mut map2)
    }

    pub fn is_more_general(&self, other: &Term) -> bool {
        match (self, other) {
            // A variable is more general than anything
            (Term::Variable(_), _) => true,

            // A function can never be more general than a variable
            (Term::Function(_), Term::Variable(_)) => false,

            // Two functions: must have same signature and recursively more general arguments
            (Term::Function(f1), Term::Function(f2)) => {
                if f1.signature != f2.signature {
                    return false;
                }
                f1.args
                    .iter()
                    .zip(&f2.args)
                    .all(|(t1, t2)| t1.is_more_general(t2))
            }
        }
    }

    pub fn is_head_function_commutative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::C)
                && !f.signature.axioms.contains(&Axioms::A){
                    return true;
                }

                false
            },
            Term::Variable(v)=>false
        }
    }

    pub fn is_head_function_associative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::A)
                    && !f.signature.axioms.contains(&Axioms::C){
                    return true;
                }

                false
            },
            Term::Variable(v)=>false
        }
    }

    pub fn is_head_function_associative_commutative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::C)
                    && f.signature.axioms.contains(&Axioms::A){
                    return true;
                }

                false
            },
            Term::Variable(v)=>false
        }
    }

    pub fn get_special_constants(&self) -> HashSet<Term> {
        match self.clone() {
            Term::Variable(v)=>{
                HashSet::new()
            },
            Term::Function(f)=>{
                if self.is_special_constant(){
                    return HashSet::from([self.clone()]);
                }

                let mut ret = HashSet::new();

                for arg in f.args{
                    ret.extend(arg.get_special_constants());
                }

                ret
            }
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