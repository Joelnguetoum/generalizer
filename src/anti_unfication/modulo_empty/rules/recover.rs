use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::terms::term::Term;

impl Configuration {
    pub fn can_apply_recover(config: &Configuration) -> bool {
        let aut = config.active[0].clone();

        if aut.t1.head_ground() != aut.t2.head_ground() {
            for aut2 in &config.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return true;
                }
            }
        }

        false
    }


    pub fn recover(config: &Configuration) -> Configuration {
        let mut new_active = config.active.clone();
        let mut new_store = config.store.clone();
        let mut new_sub = config.sub.clone();

        let aut = new_active.remove(0);

        let mut sub = Substitution::new();
        for aut2 in &config.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                sub.insert(&aut.x, &Term::Variable(aut2.x.clone()));
            }
        }

        Configuration::new(new_active,new_store,new_sub)
    }

}