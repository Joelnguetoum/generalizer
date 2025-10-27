use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::{Function, Signature};
use crate::terms::term::Term;


impl Configuration {
    pub fn can_apply_constrained_solve(&self) -> bool {
        let aut = self.active[0].clone();

        if (aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature())
            && aut.t1.get_special_constants().is_empty()
            && aut.t2.get_special_constants().is_empty()
        {
            for aut2 in &self.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return false;
                }
            }
            return true;
        }

        false
    }


    pub fn constrained_solve(&self) -> Result<Configuration, ConfigurationError>  {
        //Redundancy????
        let mut new_active = self.active.clone();
        let mut new_store = self.store.clone();
        let mut new_sub = self.sub.clone();

        let aut = new_active.remove(0);

        new_store.push(aut);

        //Configuration::new(new_active,new_store,new_sub,self.x0.clone(),self.update_history("Solve"))
        Ok(self.create_new_config(new_active, new_store, new_sub, &Rule::ConstrainedSolve))
    }



}