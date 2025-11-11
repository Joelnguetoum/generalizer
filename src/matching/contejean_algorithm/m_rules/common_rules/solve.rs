use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl  MConfiguration {

    pub fn can_apply_solve(&self)->bool{
        let problem = self.u[0].clone();


        if let (Term::Variable(x),s) = (problem.0,problem.1){

            // We check that x do appear neither in P, nor in S

            for (y,t) in self.s.iter(){
                if x==*y{
                    return false;
                }
            }

            for (x2,f,y,t) in self.p.iter() {
                if x == *x2{
                    return false;
                }
            }
        }

        true
    }


    pub fn solve(&self)->Result<Vec<MConfiguration>,MatchingError>{
        let new_y = self.y.clone();
        let mut new_u = self.u.clone();
        let new_p = self.p.clone();
        let mut new_s = self.s.clone();


        let problem = new_u.remove(0);

        match (problem.0, problem.1){
            (Term::Variable(x), s) =>{
                //let v = Term::Variable(x);
                new_s.insert(0, (x, s))
            },
            _ =>{
                return Err(MatchingError::InvalidRuleApplication);
            }
        }

        let mconf = MConfiguration::new(new_y, new_u, new_p, new_s);

        Ok(vec![mconf])
    }

}