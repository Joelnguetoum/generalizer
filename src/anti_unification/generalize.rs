use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;

use crate::anti_unification::generalizer::generalizer::Generalizer;

/*  */
impl GeneralisationProcess {
    pub fn generalize(&mut self, alpuente: bool, verbose: bool) -> Vec<Generalizer> {

        while let Some(config) = self.unsolved_configurations.pop_back() {

            self.process_configuration(config,false,alpuente,verbose,false);

        }



        self.to_generalizers()
    }

}




