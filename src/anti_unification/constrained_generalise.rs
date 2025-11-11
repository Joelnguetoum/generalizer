
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generaliser::generaliser::Generaliser;



impl GeneralisationProcess {

    pub fn constrained_generalise(&mut self,alpuente:bool,verbose:bool) -> Result<Vec<Generaliser>, ConfigurationError> {
        while let Some(config) = self.unsolved_configurations.pop() {

            /*Optimisation*/
            if config.can_apply_greedy_solve_fail(){
                continue;
            }

            self.process_configuration(config,true,alpuente,verbose);

            /*RETURN A SOLUTION AS SOON AS */
            if !self.solved_configurations.is_empty() {
                println!("Solved configuration successfully.");
                return Ok(self.to_generalisers())
            }
        }

        if self.solved_configurations.is_empty() {
            Err(ConfigurationError::ConstrainedGeneralisationFailed)
        }
        else{
            Ok(self.to_generalisers())
        }

    }



}

