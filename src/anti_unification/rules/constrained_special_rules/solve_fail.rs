use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::function::Signature;

impl Configuration {

    pub fn can_apply_solve_fail(&self) -> bool{
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
        {
            if !aut.t1.get_special_constants().is_empty() || !aut.t2.get_special_constants().is_empty(){
                return true;
            }
        }

        false
    }

}