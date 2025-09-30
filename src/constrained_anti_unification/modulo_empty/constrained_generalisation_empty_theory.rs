use crate::configuration::configuration::Configuration;
use crate::generaliser::generaliser::Generaliser;
use crate::terms::function::Signature;
use crate::terms::term::Term;

pub fn constrained_generalisation_empty_theory(t1: &Term, t2: &Term) -> Option<Generaliser>{

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