use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::configuration::history::History;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;



impl Configuration {
    pub fn can_apply_decompose_u(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_has_unit()
        {
            return true;
        }
        false
    }

    pub fn generic_decompose(&self)->Result<Vec<Configuration>, ConfigurationError>{

        if self.can_apply_decompose(){
            self.decompose()
        }
        else if self.can_apply_decompose_c(){
            self.decompose_c()
        }
        else if self.can_apply_decompose_a(){
            self.decompose_a()
        }
        else if self.can_apply_decompose_ac(){
             self.decompose_ac()
        }
        else{
            Err(ConfigurationError::RuleApplicationError)
        }

    }
    pub fn decompose_u(&self) -> Result<Vec<Configuration>, ConfigurationError> {

        //println!("Decompose u");

        let mut result_confs = Vec::new();

        //We begin by the regular decomposition

        result_confs.extend(self.generic_decompose()?);

        //Expansions and then decompostions
        let expand_confs = self.expand_same()?;

        for conf in expand_confs {

            //result_confs.extend(conf.generic_decompose()?);
            result_confs.extend(conf.generic_decompose()?);
        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}