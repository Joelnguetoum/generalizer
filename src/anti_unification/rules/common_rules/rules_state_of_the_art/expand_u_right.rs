use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::rules::rule::Rule;
use crate::terms::function::Function;
use crate::terms::term::Term;

impl Configuration{
    pub fn can_apply_expand_u_right(&self)->bool{
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_has_unit()
        {
            return true;
        }
        false
    }



    pub fn expand_u_right(&self) -> Result<Vec<Configuration>, ConfigurationError>{

       // println!("Expand u right");

        let mut new_active = self.active.clone();

        let aut = new_active.remove(0);

        let u_f = aut.t1.head_symbol_signature().get_unit();

        match (aut.t1.clone(), aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {


                //Return config 1
                let mut new_active1 = new_active.clone();
                let mut new_store1 = self.store.clone();
                let mut new_sub1 = self.sub.clone();


                let t_right_1 = Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![u_f.clone(), aut.t2.clone()]));

                let new_aut1 = AUT::new(aut.x.clone(), aut.t1.clone(),t_right_1.clone());

                new_active1.insert(0,new_aut1);


                let conf1 = self.create_new_config(new_active1, new_store1, new_sub1, &Rule::ExpandURight);


                //Return config 2
                let mut new_active2 = new_active.clone();
                let mut new_store2 = self.store.clone();
                let mut new_sub2 = self.sub.clone();


                let t_left_2 = Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![aut.t2.clone(), u_f.clone()]));

                let new_aut2 = AUT::new(aut.x.clone(), aut.t1.clone(),t_left_2.clone());

                new_active2.insert(0,new_aut2);

                let conf2 = self.create_new_config(new_active2, new_store2, new_sub2, &Rule::ExpandURight);

                Ok(vec![conf1, conf2])
            },
            _ => Err(ConfigurationError::NonGroundTerm)
        }
    }
}
