
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;




impl Configuration {
    pub fn can_apply_expand_u_both_decompose(&self) -> bool {
        self.can_apply_expand_u_left() || self.can_apply_expand_u_right()
    }

    pub fn expand_u_both_decompose(&self) -> Result<Vec<Configuration>, ConfigurationError> {

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