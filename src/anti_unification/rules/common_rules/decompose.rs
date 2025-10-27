use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::configuration::history::History;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;



impl Configuration {
    pub fn can_apply_decompose(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && !aut.t1.is_head_function_commutative()
            && !aut.t1.is_head_function_associative()
            && !aut.t1.is_head_function_associative_commutative()
            {
                //If arity mismatch, return false
                if let (Term::Function(f1),Term::Function(f2)) = (aut.t1.clone(), aut.t2.clone()) {
                    if f1.args.len() != f2.args.len(){
                        return false;
                    }
                }

            return true;
        }
        false
    }
    pub fn decompose(&self) -> Result<Vec<Configuration>, ConfigurationError> {


        // println!("decompose");

        let mut new_active = self.active.clone();
        let mut new_store = self.store.clone();
        let mut new_sub = self.sub.clone();


        //Since this is decompose, we remove the concerned AUT
        let aut = new_active.remove(0);

        match (&aut.t1.clone(), &aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {
                let mut sub = Substitution::new();
                let mut sub_term_args = Vec::new();

                for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                    let x = Variable::fresh_variable();

                    sub_term_args.push(Term::Variable(x.clone()));

                    let new_aut = AUT::new(x.clone(), t1.clone(), t2.clone());

                    new_active.insert(0,new_aut);
                }
                sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
                new_sub.push(sub);

                //Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose"))
                let conf = self.create_new_config(new_active, new_store, new_sub, &Rule::Decompose);
                Ok(vec![conf])
            },
            _ => Err(ConfigurationError::NonGroundTerm)
        }
    }

}