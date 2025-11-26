



use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;


impl Configuration {
    pub fn can_apply_decompose_a(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_associative()
            && !aut.t2.is_head_function_commutative()
        {
            return true;
        }
        false
    }


    pub fn decompose_a(&self) -> Result<Vec<Configuration>, ConfigurationError> {
        //println!("decompose_a");

        let mut return_configurations = Vec::new();

        let mut active = self.active.clone();

        //Since this is decompose, we remove the concerned AUT
        let aut = active.remove(0);

        //match (aut.t1.clone().assoc_flatten(), aut.t2.clone().assoc_flatten()) {
        match (aut.t1.clone(), aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {

                for (t1,t1_prime,t2,t2_prime) in Self::assoc_dec_groups(&f1.signature,&f1.args, &f2.args) {
                    let mut new_active = active.clone();
                    let new_store = self.store.clone();
                    let mut new_sub = self.sub.clone();





                    let (x1, x2, aut1, aut2) = self.with_fresh_aut_pair(
                        t1, t1_prime, t2, t2_prime
                    );

                    new_active.insert(0,aut1.clone());
                    new_active.insert(0,aut2.clone());

                    let mut sub = Substitution::new();
                    let sub_term_args = vec![Term::Variable(x1), Term::Variable(x2)];

                    sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
                    new_sub.push(sub);

                    //let conf = Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose_A"));
                    let conf = self.create_new_config(new_active, new_store, new_sub, &Rule::DecomposeA);
                    return_configurations.push(conf);
                }

                return Ok(return_configurations);
            },
            _ => Err(ConfigurationError::NonGroundTerm)
        }
    }


    pub fn assoc_dec_groups(sig: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
        let mut quadruples = Vec::new();

        //Left
        for i in 0..args1.len()-1 {
            let t1 = Term::assoc_wrap(sig,&Vec::from(&args1[0..=i]));//.to_associative_form();
            let t1_prime = args2[0].clone();

            let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[i+1..]));//.to_associative_form();
            let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[1..]));//.to_associative_form();

            quadruples.push((t1,t1_prime,t2,t2_prime));
        }

        //Right
        for i in 0..args2.len()-1 {
            let t1 = args1[0].clone();
            let t1_prime = Term::assoc_wrap(sig,&Vec::from(&args2[0..=i]));//.to_associative_form();

            let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[1..]));//.to_associative_form();
            let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[i+1..]));//.to_associative_form();


            quadruples.push((t1,t1_prime,t2,t2_prime));
        }



        quadruples
    }

}


impl Term{
    pub fn assoc_flatten(&self)->Term{

        match self.clone(){
            Term::Variable(_x) => self.clone(),
            Term::Function(f)=>{
                let mut new_args = Vec::new();
                for arg in f.args{
                    match arg.clone(){
                        Term::Variable(_x) => {new_args.push(arg);},
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

    pub fn assoc_wrap(sig: &FunctionSignature,args: &Vec<Term>)->Term{
        if sig.axioms.contains(&Axioms::A) && args.len() == 1{
            return args[0].clone();
        }

        Term::Function(Function::new(sig,args))
    }
}

