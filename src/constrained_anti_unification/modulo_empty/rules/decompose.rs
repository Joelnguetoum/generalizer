use crate::configuration::aut::AUT;
use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;


/*
impl Configuration {
    pub fn can_apply_decompose(config: &Configuration) -> bool {
        let aut = config.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground() {
            return true;
        }
        false
    }
    pub fn decompose(config: &Configuration) -> Configuration {
        let mut new_active = config.active.clone();
        let mut new_store = config.store.clone();
        let mut new_sub = config.sub.clone();


        //Since this is decompose, we remove the concerned AUT
        let aut = new_active.remove(0);

        match (aut.t1.clone(), aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {
                let mut sub = Substitution::new();
                let mut sub_term_args = Vec::new();

                for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                    let x = Variable::fresh_variable();

                    sub_term_args.push(Term::Variable(x.clone()));

                    let new_aut = AUT::new(x.clone(), t1.clone(), t2.clone());

                    new_active.push(new_aut);
                }
                sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_ground(), &sub_term_args)));
                new_sub.push(sub);

                Configuration::new(new_active, new_store,new_sub)
            },
            _ => panic!("Genereralisation of a non-gound term")
        }
    }

}

 */