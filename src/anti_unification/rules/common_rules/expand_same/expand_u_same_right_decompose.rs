
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;




impl Configuration {
    pub fn can_apply_expand_u_same_right_decompose(&self) -> bool {
        self.can_apply_expand_u_same_right()
    }

    pub fn expand_u_same_right_decompose(&self) -> Result<Vec<Configuration>, ConfigurationError> {

        let mut result_confs = Vec::new();

        //Expansions
        let expand_same_right_confs = self.expand_u_same_right()?;



        //And then decomposition
        for conf in expand_same_right_confs {

            result_confs.extend(conf.generic_decompose()?);

        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}