use std::fmt;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, Debug)]
pub struct Generaliser{
    pub t: Term,
    pub sub1: Substitution,
    pub sub2: Substitution,
}

impl Generaliser{
    pub fn new(t: &Term, sub1: &Substitution, sub2: &Substitution) -> Self {
        Self{t: t.clone(), sub1: sub1.clone(), sub2: sub2.clone()}
    }
}

impl fmt::Display for Generaliser{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generaliser: {} \n σ1 = {} \n σ2 = {}",self.t,self.sub1,self.sub2 )
    }
}
