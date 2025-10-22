use std::collections::{HashMap, HashSet};
use crate::configuration::configuration::Configuration;
use crate::configuration::history::History;
use crate::generaliser::generaliser::Generaliser;
use crate::generaliser::minimise::minimise_ac;
use crate::terms::function::{Axioms, Function, FunctionSignature};
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


    pub fn to_generalisers(&self) -> Vec<Generaliser> {
        let generalisers: Vec<Generaliser> = self
            .solved_configurations
            .iter()
            .map(|conf| {
                let  g = conf.to_generaliser();
                g
            })
            .collect();

        let mut filtered = minimise_ac(generalisers);

        filtered
    }

}


