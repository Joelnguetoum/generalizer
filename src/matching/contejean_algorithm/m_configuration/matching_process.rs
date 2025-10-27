use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

#[derive(Debug)]
pub struct MatchingProcess {
    pub solved_configurations: Vec<MConfiguration>,
    pub unsolved_configurations: Vec<MConfiguration>
}


impl MatchingProcess {


    pub fn init_process(t1: &Term, t2: &Term) -> MatchingProcess {
        let conf = MConfiguration::init_m_conf(t1,t2);

        MatchingProcess{solved_configurations: Vec::new(),unsolved_configurations:vec![conf]}
    }


    pub fn process_m_configuration(&mut self, config: MConfiguration) {
        // Check if configuration is already solved
        if config.U.is_empty() {
            self.solved_configurations.push(config);
            return;
        }


        // Try to apply m_rules
        match config.applicable_rule() {
            Some(rule) => {
                // println!("Rule applicable {:?}",rule);
                match config.apply_rule(rule) {
                    Ok(new_configs) => {
                        self.unsolved_configurations.extend(new_configs);
                        // println!("current number of unsolved configurations: {}", self.unsolved_configurations.len());
                    }
                    Err(e) => {
                        //If there is an error, just drop the configuration
                        //Maybe in the case of Fail, print the failure??
                    }
                }
            }
            None => {
                // If no rule applies, drop the configuration

                //panic!("No rule applied to the configuration {:?}: please complete the set of m_rules",config.clone());
            }
        }
    }
}