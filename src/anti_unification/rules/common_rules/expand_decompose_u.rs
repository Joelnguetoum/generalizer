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
    pub fn can_apply_expand_decompose_u(&self) -> bool {
        self.can_apply_expand_u_left() || self.can_apply_expand_u_right()
    }

    pub fn expand_decompose_u(&self) -> Result<Vec<Configuration>, ConfigurationError> {



        let mut result_confs = Vec::new();


        //Expansions and then decompostions

        if self.can_apply_expand_u_left(){
            let expand_left_confs = self.expand_u_left()?;

            for conf in expand_left_confs {

                //result_confs.extend(conf.generic_decompose()?);
                result_confs.extend(conf.generic_decompose()?);
            }
        }

        if self.can_apply_expand_u_right(){
            let expand_right_confs = self.expand_u_right()?;

            for conf in expand_right_confs {

                //result_confs.extend(conf.generic_decompose()?);
                result_confs.extend(conf.generic_decompose()?);
            }
        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}