
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;




impl Configuration {
    pub fn can_apply_decompose_acu(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_associative_commutative()
            && !aut.t1.is_head_function_has_unit()
        {
            return true;
        }
        false
    }


    pub fn decompose_acu(&self) -> Result<Vec<Configuration>, ConfigurationError> {

        //println!("Decompose u");

        let mut result_confs = Vec::new();

        //We begin by the regular decomposition

        result_confs.extend(self.decompose_ac()?);

        //Expansions and then decompostions
        let expand_confs = self.expand_same_both()?;

        for conf in expand_confs {

            //result_confs.extend(conf.generic_decompose()?);
            result_confs.extend(conf.decompose_ac()?);
        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}