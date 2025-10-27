use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {


    pub fn can_apply_merge_s(&self)-> bool{
        let problem = self.U[0].clone();

        if let (Term::Variable(x),s) = (problem.0,problem.1){
            for (y,s_prime) in self.S.iter() {
                if x == *y && s == *s_prime{
                    return true;
                }
            }
        }

        false
    }


    pub fn merge_s(&self)->Result<Vec<MConfiguration>,MatchingError>{
        let new_y = self.y.clone();
        let mut new_U = self.U.clone();
        let new_P = self.P.clone();
        let new_S = self.S.clone();


        let problem = new_U.remove(0);

        let mconf = MConfiguration::new(new_y,new_U,new_P,new_S);

        Ok(vec![mconf])
    }

}