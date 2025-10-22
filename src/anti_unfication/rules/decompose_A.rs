



use crate::configuration::aut::AUT;
use crate::configuration::configuration::Configuration;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;


impl Configuration {
    pub fn can_apply_decompose_a(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground()
            && aut.t1.is_head_function_associative()
            && !aut.t2.is_head_function_commutative() {
            return true;
        }
        false
    }
    pub fn decompose_a(&self) -> Vec<Configuration> {
        //println!("decompose_a");

        let mut return_configurations = Vec::new();

        let mut active = self.active.clone();

        //Since this is decompose, we remove the concerned AUT
        let aut = active.remove(0);

        match (aut.t1.clone().assoc_flatten(), aut.t2.clone().assoc_flatten()) {
            (Term::Function(f1), Term::Function(f2)) => {

                for (t1,t1_prime,t2,t2_prime) in Self::assoc_dec_groups(&f1.signature,&f1.args, &f2.args) {
                    let mut new_active = active.clone();
                    let mut new_store = self.store.clone();
                    let mut new_sub = self.sub.clone();

                    let mut sub = Substitution::new();
                    let mut sub_term_args = Vec::new();

                    let x1 = Variable::fresh_variable();
                    let x2 = Variable::fresh_variable();



                    let aut1 = AUT::new(x1.clone(), t1.clone(), t1_prime.clone());
                    let aut2 = AUT::new(x2.clone(), t2.clone(), t2_prime.clone());

                    //println!("{}",aut1);
                    //println!("{}",aut2);

                    new_active.push(aut1.clone());
                    new_active.push(aut2.clone());

                    sub_term_args.push(Term::Variable(x1.clone()));
                    sub_term_args.push(Term::Variable(x2.clone()));
                    sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_ground(), &sub_term_args)));
                    new_sub.push(sub);

                    let conf = Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose_A"));
                    return_configurations.push(conf);
                }

                return return_configurations;
            },
            _ => panic!("Generalisation of a non-ground term")
        }
    }


    pub fn assoc_dec_groups(sig: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
        let mut quadruples = Vec::new();

        //Left
        for i in 0..args1.len()-1 {
            let t1 = Term::assoc_wrap(sig,&Vec::from(&args1[0..=i]));
            let t1_prime = args2[0].clone();

            let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[i+1..]));
            let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[1..]));

            quadruples.push((t1,t1_prime,t2,t2_prime));
        }

        //Right
        for i in 0..args2.len()-1 {
            let t1 = args1[0].clone();
            let t1_prime = Term::assoc_wrap(sig,&Vec::from(&args2[0..=i]));

            let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[1..]));
            let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[i+1..]));


            quadruples.push((t1,t1_prime,t2,t2_prime));
        }



        quadruples
    }

}


impl Term{
    pub fn assoc_flatten(&self)->Term{

        match self.clone(){
            Term::Variable(x) => self.clone(),
            Term::Function(f)=>{
                let mut new_args = Vec::new();
                for arg in f.args{
                    match arg.clone(){
                        Term::Variable(x) => {new_args.push(arg);},
                        Term::Function(g)=>{
                            if g.signature== f.signature{
                                //First we flatten it
                                let new_arg = arg.assoc_flatten();

                                //Then we take its arguments
                                if let Term::Function(new_g) = new_arg{
                                    new_args.extend(new_g.args.clone());
                                }
                            }
                            else{
                                new_args.push(arg);
                            }
                        }
                    }

                }
                Term::Function(Function::new(&f.signature, &new_args))
            }


        }


    }

    pub fn assoc_unflatten(&self)->Term{
        todo!()
    }
    pub fn assoc_wrap(sig: &FunctionSignature,args: &Vec<Term>)->Term{
        if sig.axioms.contains(&Axioms::A) && args.len() == 1{
            return args[0].clone();
        }

        Term::Function(Function::new(sig,args))
    }
}

