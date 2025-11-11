
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;




impl Configuration {

    pub fn generic_decompose(&self)->Result<Vec<Configuration>, ConfigurationError>{

        if self.relaxed_can_apply_decompose(){
            self.decompose()
        }
        else if self.relaxed_can_apply_decompose_a(){
            self.decompose_a()
        }
        else if self.relaxed_can_apply_decompose_c(){
            self.decompose_c()
        }
        else if self.relaxed_can_apply_decompose_ac(){
             self.decompose_ac()
        }

        else{
            Err(ConfigurationError::RuleApplicationError)
        }

    }


}