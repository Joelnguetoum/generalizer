
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generaliser::generaliser::Generaliser;


impl GeneralisationProcess {

    pub fn constrained_generalise(&mut self,alpuente:bool,verbose:bool,greedy_fail:bool) -> Result<Vec<Generaliser>, ConfigurationError> {

        while let Some(config) = self.unsolved_configurations.pop_back() {

            /*RETURN A SOLUTION AS SOON AS ONE IS FOUND*/
            if !self.solved_configurations.is_empty() {
                //println!("Solved configuration successfully.");
                return Ok(self.to_generalisers())
            }

            self.process_configuration(config,true,alpuente,verbose,greedy_fail);


        }

        if self.solved_configurations.is_empty() {
            Err(ConfigurationError::ConstrainedGeneralisationFailed)
        }
        else{
            Ok(self.to_generalisers())
        }



    }

}


/*

impl GeneralisationEngine{
    pub fn constrained_generalise(&mut self,alpuente: bool,verbose: bool) -> Result<Vec<Generaliser>, ConfigurationError>{

        self.run(true,alpuente,verbose);

        let generalisers = self.to_generalisers();

        if generalisers.len() == 0{
            Err(ConfigurationError::ConstrainedGeneralisationFailed)
        }
        else{
            Ok(self.to_generalisers())
        }

    }
}

 */
