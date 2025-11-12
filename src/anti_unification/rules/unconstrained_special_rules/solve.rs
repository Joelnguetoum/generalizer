use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;



impl Configuration {
    pub fn can_apply_solve(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
            && !aut.t1.is_head_function_has_unit()
            && !aut.t2.is_head_function_has_unit(){
            for aut2 in &self.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return false;
                }
            }
            return true;
        }

        false
    }


    pub fn solve(&self) -> Result<Configuration, ConfigurationError> {
        let mut new_active = self.active.clone();
        let mut new_store = self.store.clone();
        let new_sub = self.sub.clone();

        let aut = new_active.remove(0);

        new_store.push(aut);

        //Configuration::new(new_active,new_store,new_sub,self.x0.clone(),self.update_history("Solve"))

        Ok(self.create_new_config(new_active, new_store, new_sub, &Rule::Solve))
    }

}