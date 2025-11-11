use std::fs;
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;

use crate::anti_unification::generaliser::generaliser::Generaliser;



impl GeneralisationProcess {
    pub fn generalise(&mut self,alpuente: bool,verbose: bool) -> Vec<Generaliser> {

        let mut ct = 0;
        let dir_step = "generaliser_steps";
        fs::remove_dir_all(dir_step).ok();
        fs::create_dir_all(dir_step).ok();

        while let Some(config) = self.unsolved_configurations.pop() {

            self.process_configuration(config,false,alpuente,verbose);

            if verbose {
                let new_step = format!("{}/step {}",dir_step, ct);
                fs::create_dir(new_step.clone()).ok();
                self.print_unsolved_configurations(&new_step);
                ct+=1;
            }

        }
        self.to_generalisers()
    }

}

