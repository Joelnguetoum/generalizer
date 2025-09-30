use crate::configuration::configuration::Configuration;
use crate::terms::function::Signature;

impl Configuration {

    pub fn can_apply_fail(config: &Configuration) -> bool{
        let aut = config.active[0].clone();

        match (aut.t1.is_special_constant(), aut.t2.is_special_constant()){
            (false,true)=> true,
            (true,false)=> true,
            (true,true)=> {
                aut.t1.head_ground() != aut.t2.head_ground()
            },
            _ => false,
        }
    }

}