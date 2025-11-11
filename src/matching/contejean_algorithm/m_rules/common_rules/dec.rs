use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    pub fn can_apply_dec(&self) -> bool {
        let pb = self.u[0].clone();



        match self.u[0].clone(){
            (Term::Function(f1), Term::Function(f2)) => {
                if f1.signature == f2.signature
                && f1.args.len() == f2.args.len()
                && !pb.0.is_head_function_commutative()
                && !pb.0.is_head_function_associative(){
                    return true;
                }
            },
            _ => {}
        }

        false
    }


    pub fn dec(&self) -> Result<Vec<MConfiguration>,MatchingError> {
            let new_y = self.y.clone();
            let mut new_u = self.u.clone();
            let new_p = self.p.clone();
            let new_s = self.s.clone();


            let problem = new_u.remove(0);

            match problem{
                (Term::Function(f1),Term::Function(f2))=>{
                    let mut new_problems = Vec::new();

                    for (p1,p2) in f1.args.iter().zip(f2.args.iter()) {
                        new_problems.push((p1.clone(),p2.clone()));
                    }

                    new_problems.extend(new_u);
                    new_u = new_problems;

                    let mconf = MConfiguration::new(new_y, new_u, new_p, new_s);

                    Ok(vec![mconf])
                },
                _ => {
                    return Err(MatchingError::InvalidRuleApplication);
                }
            }
    }

}