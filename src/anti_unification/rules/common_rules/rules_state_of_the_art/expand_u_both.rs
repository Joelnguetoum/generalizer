

use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;




impl Configuration {
    pub fn can_apply_expand_u_both(&self) -> bool {
        self.can_apply_expand_u_left() || self.can_apply_expand_u_right()
    }

    pub fn expand_u_both(&self) -> Result<Vec<Configuration>, ConfigurationError> {

        let mut result_confs = Vec::new();

        //Expansions and then decompostions

        if self.can_apply_expand_u_left(){

            result_confs.extend(self.expand_u_left()?);
        }

        if self.can_apply_expand_u_right(){

            result_confs.extend(self.expand_u_right()?);
        }

        //Return the resulting configurations
        Ok(result_confs)
    }

}