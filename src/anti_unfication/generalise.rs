use std::collections::HashSet;
use crate::configuration::configuration::Configuration;
use crate::configuration::generalisation_process::GeneralisationProcess;
use crate::configuration::history::History;
use crate::generaliser::generaliser::Generaliser;
use crate::terms::function::Signature;
use crate::terms::term::Term;


impl GeneralisationProcess {
    pub fn generalise(&mut self) -> Vec<Generaliser> {

        while let Some(config) = self.unsolved_configurations.pop() {
            if config.active.is_empty(){
                self.solved_configurations.push(config.clone());
                continue;
            }

            if Configuration::can_apply_decompose(&config) {
                let new_config = Configuration::decompose(&config);
                self.unsolved_configurations.push(new_config.clone());
                continue;
            }

            if Configuration::can_apply_decompose_c(&config) {
                let new_configs = Configuration::decompose_c(&config);
                self.unsolved_configurations.extend(new_configs.clone());
                continue;
            }

            if Configuration::can_apply_decompose_a(&config) {
                let new_configs = Configuration::decompose_a(&config);
                self.unsolved_configurations.extend(new_configs.clone());
                continue;
            }

            if Configuration::can_apply_decompose_ac(&config) {
                let new_configs = Configuration::decompose_ac(&config);
                self.unsolved_configurations.extend(new_configs.clone());
                continue;
            }


            if Configuration::can_apply_solve(&config) {
                let new_config = Configuration::solve(&config);
                self.unsolved_configurations.push(new_config.clone());
                continue;
            }

            if Configuration::can_apply_recover(&config) {
                let new_config = Configuration::recover(&config);
                self.unsolved_configurations.push(new_config.clone());
                continue;
            }

        }

        //println!("{:?}",self);

        self.to_generalisers()
    }

}

