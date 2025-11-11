use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::anti_unification::generaliser::minimise::minimise_ac;
use crate::terms::term::Term;

#[derive(Debug)]
pub struct GeneralisationProcess {
    pub solved_configurations: Vec<Configuration>,
    pub unsolved_configurations: Vec<Configuration>
}

impl GeneralisationProcess {
    pub fn new(conf: &Configuration) -> Self {
        Self{solved_configurations: Vec::new(),unsolved_configurations:vec![conf.clone()]}
    }

    pub fn init_process(t1: &Term, t2: &Term) -> GeneralisationProcess {
        let conf = Configuration::init_conf(t1,t2);

        GeneralisationProcess::new(&conf)
    }

    pub fn process_configuration(&mut self, config: Configuration,is_constrained_anti_unification: bool,alpuente: bool,verbose: bool) {
        // Check if configuration is already solved
        if config.active.is_empty() {
            self.solved_configurations.push(config);
            return;
        }


        // Try to apply m_rules
        match config.applicable_rule(is_constrained_anti_unification,alpuente) {
            Some(rule) => {
                if verbose {
                    println!("Rule applicable {:?}",rule);
                    println!("current number of unsolved configurations: {}", self.unsolved_configurations.len());
                    println!("Current Configuration:  {}",config.clone());

                }

                match config.apply_rule(rule) {
                    Ok(new_configs) => {
                        /* BFS
                        let mut temps = new_configs.clone();
                        for conf in &self.unsolved_configurations {
                            temps.push(conf.clone());
                        }
                        self.unsolved_configurations = temps;

                        */


                        /*DFS */
                        self.unsolved_configurations.extend(new_configs);



                    }
                    Err(e) => {
                        //If there is an error, just drop the configuration
                        //Maybe in the case of SolveFail, print the failure??
                    }
                }
            }
            None => {
                // If no rule applies, panic!
                panic!("No rule applied to the configuration {}: please complete the set of m_rules",config.clone());
            }
        }
    }


    pub fn to_generalisers(&self) -> Vec<Generaliser> {
        let generalisers: Vec<Generaliser> = self
            .solved_configurations
            .iter()
            .map(|conf| {
                let  g = conf.to_generaliser();
                g
            })
            .collect();

        //let mut filtered = minimise_ac(generalisers);

        //filtered
        generalisers
    }
    
    pub fn print_unsolved_configurations(&self,dir: &str) {
        for (ct,config) in self.unsolved_configurations.iter().enumerate() {
            let file = format!("{}/conf {}",dir,ct);
            let mut conf = config.clone();
            conf.history.add_config(&config,"");

            conf.history.create_computation_graph(&file).unwrap()
        }
    }

}


