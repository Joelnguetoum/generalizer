
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;

use crate::anti_unification::generaliser::generaliser::Generaliser;



impl GeneralisationProcess {
    pub fn generalise(&mut self) -> Vec<Generaliser> {
        while let Some(config) = self.unsolved_configurations.pop() {

            self.process_configuration(config,false);
        }
        self.to_generalisers()
    }

}

