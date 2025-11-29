use std::collections::VecDeque;
use colored::Colorize;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalizer::generalizer::Generalizer;
use crate::anti_unification::generalizer::minimise::minimise_ac;
use crate::anti_unification::rules::rule::Rule;
use crate::terms::term::Term;

#[derive(Debug)]
pub struct GeneralisationProcess {
    pub solved_configurations: Vec<Configuration>,
    pub unsolved_configurations: VecDeque<Configuration>
}

impl GeneralisationProcess {
    pub fn new(conf: &Configuration) -> Self {
        Self{solved_configurations: Vec::new(),unsolved_configurations:VecDeque::from([conf.clone()])}
    }

    pub fn init_process(t1: &Term, t2: &Term) -> GeneralisationProcess {
        let conf = Configuration::init_conf(t1,t2);

        GeneralisationProcess::new(&conf)
    }


    pub fn process_configuration(&mut self, config: Configuration,is_constrained_anti_unification: bool,alpuente: bool,verbose: bool, greedy_fail: bool) {
        // Check if configuration is already solved
        if config.active.is_empty() {
            self.solved_configurations.push(config);
            return;
        }

        let rules = config.get_applicable_rules_first_aut(is_constrained_anti_unification,alpuente,greedy_fail);



        if rules.contains(&Rule::SolveFail) && !greedy_fail {
            if verbose {
                println!("Rule applicable {}","SolveFail".red());
                println!("Current Configuration:  {}",config.clone());
                println!("========================================");
            }
            return;
        }


        /*  */
        if rules.contains(&Rule::GreedySolveFail)&& greedy_fail {
            if verbose {
                println!("Rule applicable {}","GreedySolveFail".red());
                println!("Current Configuration:  {}",config.clone());
                println!("========================================");
            }
            return;
        }


        /*Optimisation
            if config.can_apply_greedy_solve_fail(){
                continue;
            }
            */

        for rule in rules {
            // Try to apply m_rules
            match config.apply_rule(rule.clone()) {
                Ok(res) => {
                    if verbose {
                        println!("Rule applicable {:?}",rule);
                        println!("current number of unsolved configurations: {}", self.unsolved_configurations.len());
                         println!("Current Configuration:  {}",config.clone());
                        println!("========================================");

                    }

                    self.unsolved_configurations.extend(res);
                    //queue.push_back(res[0].clone());

                }
                Err(_) => {
                    //If there is an error, just drop the configuration
                    //Maybe in the case of SolveFail, print the failure??
                }
            }
        }

    }




    pub fn to_generalisers(&self) -> Vec<Generalizer> {
        let generalisers: Vec<Generalizer> = self
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
    #[allow(dead_code)]
    pub fn to_generalisers_with_minimise(&self) -> Vec<Generalizer> {
        let generalisers: Vec<Generalizer> = self
            .solved_configurations
            .iter()
            .map(|conf| {
                let  g = conf.to_generaliser();
                g
            })
            .collect();

        minimise_ac(generalisers)
    }
    #[allow(dead_code)]
    pub fn print_unsolved_configurations(&self,dir: &str) {
        for (ct,config) in self.unsolved_configurations.iter().enumerate() {
            let file = format!("{}/conf {}",dir,ct);
            let mut conf = config.clone();
            conf.history.add_config(&config,"");

            conf.history.create_computation_graph(&file).unwrap()
        }
    }

}


