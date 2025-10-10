use crate::configuration::configuration::Configuration;
use crate::terms::function::Signature;

impl Configuration {

    pub fn can_apply_fail(config: &Configuration) -> bool{
        let aut = config.active[0].clone();

        if aut.t1.head_ground() != aut.t2.head_ground()
        {
            if !aut.t1.get_special_constants().is_empty() || !aut.t2.get_special_constants().is_empty(){
                return true;
            }
        }

        false
    }

}