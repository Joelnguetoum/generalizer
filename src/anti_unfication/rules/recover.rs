use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::terms::term::Term;

impl Configuration {
    pub fn can_apply_recover(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() != aut.t2.head_ground() {
            for aut2 in &self.store {
                if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                    return true;
                }
            }
        }

        false
    }


    pub fn recover(&self) -> Configuration {
        let mut new_active = self.active.clone();
        let mut new_store = self.store.clone();
        let mut new_sub = self.sub.clone();

        let aut = new_active.remove(0);

        let mut sub = Substitution::new();
        for aut2 in &self.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                sub.insert(&aut.x, &Term::Variable(aut2.x.clone()));
            }
        }
        new_sub.push(sub);
        Configuration::new(new_active,new_store,new_sub,self.x0.clone(),self.update_history("Recover"))
    }

}