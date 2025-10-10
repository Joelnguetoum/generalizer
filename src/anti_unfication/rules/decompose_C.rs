


use crate::configuration::aut::AUT;
use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;



impl Configuration {
    pub fn can_apply_decompose_c(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground()
            && aut.t1.is_head_function_commutative()
            && !aut.t2.is_head_function_associative() {
            return true;
        }
        false
    }
    pub fn decompose_c(&self) -> Vec<Configuration> {
       // println!("decompose_c");

        let mut new_active = self.active.clone();

        //Since this is decompose, we remove the concerned AUT
        let aut = new_active.remove(0);

        match (aut.t1.clone(), aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {


                //Return config 1
                let mut new_active1 = new_active.clone();
                let mut new_store1 = self.store.clone();
                let mut new_sub1 = self.sub.clone();

                let mut sub1 = Substitution::new();
                let mut sub_term_args1 = Vec::new();

                for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                    let x1 = Variable::fresh_variable();

                    sub_term_args1.push(Term::Variable(x1.clone()));

                    let new_aut1 = AUT::new(x1.clone(), t1.clone(), t2.clone());

                    new_active1.push(new_aut1);
                }
                sub1.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_ground(), &sub_term_args1)));
                new_sub1.push(sub1);

                let conf1 = Configuration::new(new_active1, new_store1,new_sub1);

                //Return config 2

                let mut new_active2 = new_active.clone();
                let mut new_store2 = self.store.clone();
                let mut new_sub2 = self.sub.clone();

                let mut sub2 = Substitution::new();
                let mut sub_term_args2 = Vec::new();

                //let mut f2_args_rev = f2.args.clone();
                //f2_args_rev.reverse();

                for (t1, t2) in f1.args.iter().zip(f2.args.iter().rev()) {
                    let x2 = Variable::fresh_variable();

                    sub_term_args2.push(Term::Variable(x2.clone()));

                    let new_aut2 = AUT::new(x2.clone(), t1.clone(), t2.clone());

                    new_active2.push(new_aut2);
                }
                sub2.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_ground(), &sub_term_args2)));
                new_sub2.push(sub2);

                let conf2 = Configuration::new(new_active2, new_store2,new_sub2);

                //return
                vec![conf1, conf2]
            },
            _ => panic!("Generalisation of a non-ground term")
        }
    }

}