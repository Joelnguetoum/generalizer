use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    pub fn can_apply_dec_c(&self) -> bool {
        let pb = self.U[0].clone();



        match self.U[0].clone(){
            (Term::Function(f1), Term::Function(f2)) => {
                if f1.signature == f2.signature
                    && f1.args.len() == f2.args.len()
                    && pb.0.is_head_function_commutative()
                    && !pb.0.is_head_function_associative(){
                    return true;
                }
            },
            _ => {}
        }

        false
    }


    pub fn dec_c(&self) -> Result<Vec<MConfiguration>,MatchingError> {

        let mut new_U = self.U.clone();



        let problem = new_U.remove(0);

        match problem{
            (Term::Function(f1),Term::Function(f2))=>{

                //Conf 1

                let mut new_U1 = new_U.clone();
                let new_y1 = self.y.clone();
                let new_P1 = self.P.clone();
                let new_S1 = self.S.clone();

                let mut new_problems1 = Vec::new();


                for (p1,p2) in f1.args.iter().zip(f2.args.iter()) {
                    new_problems1.push((p1.clone(),p2.clone()));
                }

                new_problems1.extend(new_U1);
                new_U1 = new_problems1;

                let mconf1 = MConfiguration::new(new_y1,new_U1,new_P1,new_S1);

                //conf2

                let mut new_U2 = new_U.clone();
                let new_y2 = self.y.clone();
                let new_P2 = self.P.clone();
                let new_S2 = self.S.clone();
                let mut new_problems2 = Vec::new();


                for (p1,p2) in f1.args.iter().zip(f2.args.iter().rev()) {
                    new_problems2.push((p1.clone(),p2.clone()));
                }

                new_problems2.extend(new_U2);
                new_U2 = new_problems2;

                let mconf2 = MConfiguration::new(new_y2,new_U2,new_P2,new_S2);

                Ok(vec![mconf1, mconf2])
            },
            _ => {
                return Err(MatchingError::InvalidRuleApplication);
            }
        }
    }

}