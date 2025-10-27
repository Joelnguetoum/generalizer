use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::function::Function;
use crate::terms::term::Term;
use crate::utils::misc::remove_first;

impl MConfiguration {


    pub fn can_apply_merge_p(&self)-> bool{
        let problem = self.U[0].clone();

        if let (Term::Variable(x),Term::Function(f)) = (problem.0.clone(),problem.1.clone()) {

            if !problem.1.is_head_function_associative() || !problem.1.is_head_function_commutative(){
                return false;
            }

            for (x_1,f_prime,y,s_prime) in self.P.iter() {
                if f.signature == *f_prime
                && f.args.contains(s_prime){
                    return true;
                }
            }
        }

        false
    }


    pub fn merge_p(&self)->Result<Vec<MConfiguration>,MatchingError>{
        let new_y = self.y.clone();
        let mut new_U = self.U.clone();
        let new_P = self.P.clone();
        let new_S = self.S.clone();


        let problem = new_U.remove(0);

        if let (Term::Variable(x),Term::Function(f)) = (problem.0.clone(),problem.1.clone()) {

            for (x1,f_prime,y,s_prime) in self.P.iter() {
                if x == *x1
                    && f.signature == *f_prime
                    && f.args.contains(s_prime){
                    let new_args  = remove_first(f.args.clone(),s_prime.clone());
                    let new_var = Term::Variable(y.clone());
                    let new_term = Term::Function(Function::new(&f.signature, &new_args));
                    new_U.insert(0,(new_var,new_term));
                    break;
                }
            }
        }

        let mconf = MConfiguration::new(new_y,new_U,new_P,new_S);

        Ok(vec![mconf])
    }



}