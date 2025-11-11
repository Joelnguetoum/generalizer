use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {


    pub fn can_apply_clash_s(&self)-> bool{
        let problem = self.u[0].clone();

        if let (Term::Variable(x),s) = (problem.0,problem.1){
            for (y,t) in self.s.iter() {
                if x == *y && s != *t{
                    return true;
                }
            }
        }

        false
    }

}