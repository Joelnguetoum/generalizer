use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    pub fn can_apply_clash_p(&self)-> bool{
        let problem = self.U[0].clone();

        if let (Term::Variable(x1),s) = (problem.0,problem.1){
            for (x2,f,y,t) in self.P.iter() {
                if x1 == *x2 && s.head_symbol_signature() != *f{
                    return true;
                }
            }
        }

        false
    }

}