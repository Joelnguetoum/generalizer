
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generaliser::generaliser::Generaliser;



impl GeneralisationProcess {

    pub fn constrained_generalise(&mut self) -> Result<Vec<Generaliser>, ConfigurationError> {
        while let Some(config) = self.unsolved_configurations.pop() {
            self.process_configuration(config,true);
        }

        if self.solved_configurations.is_empty() {
            Err(ConfigurationError::ConstrainedGeneralisationFailed)
        }
        else{
            Ok(self.to_generalisers())
        }

    }



}

