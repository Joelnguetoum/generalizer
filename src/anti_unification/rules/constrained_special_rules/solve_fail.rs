use crate::anti_unification::configuration::configuration::Configuration;

impl Configuration {
    #[allow(dead_code)]
    pub fn can_apply_solve_fail(&self) -> bool{
        //let aut = self.active[0].clone();
        for aut in self.active.iter(){
            if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
                && !aut.t1.is_head_function_has_unit()
                && !aut.t2.is_head_function_has_unit()
            {
                if !aut.t1.get_special_constants().is_empty() || !aut.t2.get_special_constants().is_empty(){
                    return true;
                }
            }
        }


        false
    }

}