use crate::anti_unification::configuration::configuration::Configuration;

impl Configuration {

    pub fn can_apply_greedy_solve_fail(&self) -> bool{
        for aut in self.active.iter(){

            if aut.t1.get_special_constants() != aut.t2.get_special_constants()
            {
                return true;
            }
        }


        false
    }

}