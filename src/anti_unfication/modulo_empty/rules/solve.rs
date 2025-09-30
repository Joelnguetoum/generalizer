use crate::configuration::aut::AUT;
use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;


impl Configuration {
    pub fn can_apply_solve(config: &Configuration) -> bool {
        let aut = config.active[0].clone();

        if aut.t1.head_ground() != aut.t2.head_ground() {
            for aut2 in &config.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return false;
                }
            }
            return true;
        }

        false
    }


    pub fn solve(config: &Configuration) -> Configuration {
        let mut new_active = config.active.clone();
        let mut new_store = config.store.clone();
        let mut new_sub = config.sub.clone();

        let aut = new_active.remove(0);

        new_store.push(aut);

        Configuration::new(new_active,new_store,new_sub)
    }

}