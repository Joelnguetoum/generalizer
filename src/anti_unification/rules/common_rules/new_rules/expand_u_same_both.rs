use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::error::ConfigurationError;


impl Configuration {
    pub fn can_apply_expand_same_both(&self) -> bool {
        self.can_apply_expand_same_left()
            || self.can_apply_expand_same_right()
            //|| self.can_apply_expand_same_right_1()
            //|| self.can_apply_expand_same_right_2()
    }




    pub fn expand_same_both(&self) -> Result<Vec<Configuration>, ConfigurationError>  {

        let mut result_confs = Vec::new();

        //We begin by the regular decomposition

        result_confs.extend(self.expand_same_left()?);


        result_confs.extend(self.expand_same_right()?);

        //Return the resulting configurations
        Ok(result_confs)
    }

}