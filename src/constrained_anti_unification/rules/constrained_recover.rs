use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::terms::function::Signature;
use crate::terms::term::Term;

impl Configuration {
    pub fn can_apply_constrained_recover(config: &Configuration) -> bool {
        let aut = config.active[0].clone();

        if (aut.t1.head_ground() != aut.t2.head_ground())&& !aut.t1.is_special_constant() && !aut.t2.is_special_constant() {
            for aut2 in &config.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return true;
                }
            }
        }

        false
    }


    pub fn constrained_recover(config: &Configuration) -> Configuration {
        //Redundancy????

        let mut new_active = config.active.clone();
        let mut new_store = config.store.clone();
        let mut new_sub = config.sub.clone();

        let aut = new_active.remove(0);

        let mut sub = Substitution::new();
        for aut2 in &config.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                sub.insert(&aut.x, &Term::Variable(aut2.x.clone()));
                new_sub.push(sub.clone());
            }
        }

        Configuration::new(new_active,new_store,new_sub)
    }



}