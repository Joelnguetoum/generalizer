
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;

use crate::terms::term::Term;



impl Configuration {
    pub fn can_apply_decompose_u(&self) -> bool {
        let aut = &self.active[0];
        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && !aut.t1.is_head_function_commutative()
            && !aut.t1.is_head_function_associative()
            && !aut.t1.is_head_function_associative_commutative()
            && aut.t1.is_head_function_has_unit()
        {
            //If arity mismatch, return false
            if let (Term::Function(f1),Term::Function(f2)) = (aut.t1.clone(), aut.t2.clone()) {
                if f1.args.len() == f2.args.len(){
                    return true;
                }
            }
        }
        false
    }

    pub fn decompose_u(&self) -> Result<Vec<Configuration>, ConfigurationError> {

        let mut result_confs = Vec::new();

        //We begin by the regular decomposition

        result_confs.extend(self.decompose()?);

        //Expansions and then decompositions

        let expand_confs = self.expand_same_both()?;

        for conf in expand_confs {

            //result_confs.extend(conf.generic_decompose()?);
            result_confs.extend(conf.decompose()?);
        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}