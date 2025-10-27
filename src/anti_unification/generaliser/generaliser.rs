use std::fmt;
use crate::anti_unification::configuration::history::History;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, Debug)]
pub struct Generaliser{
    pub t: Term,
    pub sub1: Substitution,
    pub sub2: Substitution,
    pub history: Option<History>,
}

impl Generaliser{
    pub fn new(t: &Term, sub1: &Substitution, sub2: &Substitution) -> Self {
        Self{t: t.clone(), sub1: sub1.clone(), sub2: sub2.clone(), history: None}
    }

    pub fn new_with_history(t: &Term, sub1: &Substitution, sub2: &Substitution,history: &History) -> Self {
        Self{t: t.clone(), sub1: sub1.clone(), sub2: sub2.clone(), history: Some(history.clone())}
    }

}

impl fmt::Display for Generaliser{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} \n σ₁ = {} \n σ₂ = {}",self.t,self.sub1,self.sub2)
        /*
        if let Some(history) = &self.history {
            write!(f, "{} \n σ₁ = {} \n σ₂ = {} \n {}",self.t,self.sub1,self.sub2,history )
        }
        else{
            write!(f, "{} \n σ₁ = {} \n σ₂ = {}",self.t,self.sub1,self.sub2)
        }
         */
    }
}
