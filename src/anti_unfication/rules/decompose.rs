use crate::configuration::aut::AUT;
use crate::configuration::configuration::Configuration;
use crate::configuration::history::History;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;



impl Configuration {
    pub fn can_apply_decompose(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground()
            && !aut.t1.is_head_function_commutative()
            && !aut.t1.is_head_function_associative()
            && !aut.t1.is_head_function_associative_commutative(){
            return true;
        }
        false
    }
    pub fn decompose(&self) -> Configuration {


        // println!("decompose");

        let mut new_active = self.active.clone();
        let mut new_store = self.store.clone();
        let mut new_sub = self.sub.clone();


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

                Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose"))
            },
            _ => panic!("Genereralisation of a non-gound term")
        }
    }

}