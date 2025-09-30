use crate::configuration::configuration::Configuration;
use crate::generaliser::generaliser::Generaliser;
use crate::terms::function::Signature;
use crate::terms::term::Term;

pub fn generalisation_empty_theory( t1: &Term, t2: &Term) -> Generaliser{

    let mut config = Configuration::init_conf(t1, t2);

    while !config.active.is_empty(){
        if Configuration::can_apply_decompose(&config){
            config = Configuration::decompose(&config);
            continue;
        }

        if Configuration::can_apply_solve(&config){
            config = Configuration::solve(&config);
            continue;
        }

        if Configuration::can_apply_recover(&config){
            config = Configuration::recover(&config);
            continue;
        }
    }

    config.to_generaliser()
}