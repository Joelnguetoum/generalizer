







use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::Function;
use crate::terms::term::Term;

use std::collections::VecDeque;

impl Configuration {
    pub fn can_apply_expand_same(&self) -> bool {
        self.can_apply_expand_same_left_1()
            || self.can_apply_expand_same_left_2()
            //|| self.can_apply_expand_same_right_1()
            //|| self.can_apply_expand_same_right_2()
    }

    pub fn can_apply_expand_same_left_1(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_has_unit()
        {
            match (aut.t1.clone(),aut.t2.clone()) {
                (Term::Function(f1),Term::Function(f2))=>{
                        if f1.args[0] != aut.t1.head_symbol_signature().get_unit()
                            && f2.args[0] != aut.t2.head_symbol_signature().get_unit()
                        {
                            return true;
                        }
                },
                _ => {}
            }
        }
        false
    }

    pub fn can_apply_expand_same_left_2(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_has_unit()
        {
            match (aut.t1.clone(),aut.t2.clone()) {
                (Term::Function(f1),Term::Function(f2))=>{

                    if f1.args[f1.args.len()-1] != aut.t1.head_symbol_signature().get_unit()
                    && f2.args[f2.args.len()-1] != aut.t2.head_symbol_signature().get_unit(){
                        return true;
                    }
                },
                _ => {}
            }
        }
        false
    }
    /*
    pub fn can_apply_expand_same_right_1(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground()
            && aut.t1.is_head_function_has_unit()
        {
            match (aut.t1.clone(),aut.t2.clone()){
                (Term::Function(f1)=>{
                    if f.args[0] != aut.t1.head_ground().get_unit()

                   {
                        return true;
                    }
                },
                _ => {}
            }
        }
        false
    }

    pub fn can_apply_expand_same_right_2(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_ground() == aut.t2.head_ground()
            && aut.t1.is_head_function_has_unit()
        {
            match aut.t2.clone(){
                Term::Function(f)=>{
                    if f.args[f.args.len()-1] != aut.t1.head_ground().get_unit(){
                        return true;
                    }
                },
                _ => {}
            }
        }
        false
    }

     */

    pub fn expand_same(&self) -> Result<Vec<Configuration>, ConfigurationError>  {

       // println!("Expand same");

        let mut new_active = self.active.clone();

        //Since this is decompose, we remove the concerned AUT
        let aut = new_active.remove(0);

        let mut result_confs = Vec::new();

        let U_f = aut.t1.head_symbol_signature().get_unit();

        match (aut.t1.clone(), aut.t2.clone()) {
            (Term::Function(f1), Term::Function(f2)) => {

                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //Expand_Same_Left_1

                if self.can_apply_expand_same_left_1(){

                    //Same as expand_u_left

                    let mut new_active1 = new_active.clone();
                    let mut new_store1 = self.store.clone();
                    let mut new_sub1 = self.sub.clone();

                    let t_left_1 = Term::Function(Function::new(&aut.t2.head_symbol_signature(), &vec![U_f.clone(), aut.t1.clone()]));

                    let new_aut1 = AUT::new(aut.x.clone(), t_left_1.clone(), aut.t2.clone());

                    new_active1.insert(0,new_aut1);

                    let conf1 = self.create_new_config(new_active1, new_store1, new_sub1, &Rule::ExpandUSame);

                    result_confs.push(conf1);
                }



                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //Expand_Same_Left_2
                if self.can_apply_expand_same_left_2(){

                    let mut new_active2 = new_active.clone();
                    let mut new_store2 = self.store.clone();
                    let mut new_sub2 = self.sub.clone();



                    let t_left_2 = Term::Function(Function::new(&aut.t2.head_symbol_signature(), &vec![aut.t1.clone(), U_f.clone()]));

                    let new_aut2 = AUT::new(aut.x.clone(), t_left_2.clone(), aut.t2.clone());

                    new_active2.insert(0,new_aut2);

                    let conf2 = self.create_new_config(new_active2, new_store2, new_sub2, &Rule::ExpandUSame);

                    result_confs.push(conf2);
                }


                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //Expand_Same_Right_1
                //if self.can_apply_expand_same_right_1(){
                if self.can_apply_expand_same_left_1(){

                    //Same as expand_u_right
                    let mut new_active3 = new_active.clone();
                    let mut new_store3 = self.store.clone();
                    let mut new_sub3 = self.sub.clone();


                    let t_right_3 = Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![U_f.clone(), aut.t2.clone()]));

                    let new_aut3 = AUT::new(aut.x.clone(), aut.t1.clone(),t_right_3.clone());

                    new_active3.insert(0,new_aut3);


                    let conf3 = self.create_new_config(new_active3, new_store3, new_sub3, &Rule::ExpandUSame);

                    result_confs.push(conf3);
                }


                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //Expand_Same_Right_2
                //if self.can_apply_expand_same_right_2(){
                if self.can_apply_expand_same_left_2(){

                    //Same as expand_u_right
                    let mut new_active4 = new_active.clone();
                    let mut new_store4 = self.store.clone();
                    let mut new_sub4 = self.sub.clone();


                    let t_left_4 = Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![aut.t2.clone(), U_f.clone()]));

                    let new_aut4 = AUT::new(aut.x.clone(), aut.t1.clone(),t_left_4.clone());

                    new_active4.insert(0,new_aut4);

                    let conf4 = self.create_new_config(new_active4, new_store4, new_sub4, &Rule::ExpandUSame);

                    result_confs.push(conf4);
                }

                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //Decompose
                /*
                let mut dec_results = Vec::new();
                if self.can_apply_decompose(){
                    dec_results = self.decompose()?;
                }
                else if self.can_apply_decompose_c(){
                    dec_results = self.decompose_c()?;
                }
                else if self.can_apply_decompose_a(){
                    dec_results = self.decompose_a()?;
                }
                else if self.can_apply_decompose_ac(){
                    dec_results = self.decompose_ac()?;
                }

                result_confs.extend(dec_results);

                 */
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //return

                Ok(result_confs)
            },
            _ => Err(ConfigurationError::NonGroundTerm)
        }
    }

}