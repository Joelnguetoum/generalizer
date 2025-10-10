use std::collections::HashSet;
use crate::configuration::configuration::Configuration;
use crate::configuration::generalisation_process::GeneralisationProcess;
use crate::generaliser::generaliser::Generaliser;
use crate::terms::function::Signature;
use crate::terms::term::Term;


impl GeneralisationProcess {
    pub fn constrained_generalise(&mut self) -> Result<Vec<Generaliser>,String> {

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

            if Configuration::can_apply_constrained_solve(&config) {
                let new_config = Configuration::constrained_solve(&config);
                self.unsolved_configurations.push(new_config.clone());
                continue;
            }

            if Configuration::can_apply_recover(&config) {
                let new_config = Configuration::constrained_recover(&config);
                self.unsolved_configurations.push(new_config.clone());
                continue;
            }

            if Configuration::can_apply_fail(&config){
                continue
            }

            //If no rule could be applied
            return Err (format!("Error; no rule applicable to the following configuration:\n {}",config));
        }

        if self.solved_configurations.is_empty() && self.unsolved_configurations.is_empty() {
            return Err("Constrained generalisation Failed \n No constrained generaliser could be found".to_string());
        }

        Ok(self.to_generalisers())
    }

}

/*
pub fn constrained_generalise(t1: &Term, t2: &Term) -> Option<Generaliser>{

    let mut config = Configuration::init_conf(t1, t2);

    while !config.active.is_empty(){
        if Configuration::can_apply_decompose(&config){
            config = Configuration::decompose(&config);
            continue;
        }

        if Configuration::can_apply_constrained_solve(&config){
            config = Configuration::solve(&config);
            continue;
        }

        if Configuration::can_apply_constrained_recover(&config){
            config = Configuration::recover(&config);
            continue;
        }

        if Configuration::can_apply_fail(&config){
            return None
        }
    }

    Some(config.to_generaliser())
}

 */