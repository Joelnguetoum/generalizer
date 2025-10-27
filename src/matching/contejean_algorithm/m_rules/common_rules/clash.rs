use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration{
    pub fn can_apply_clash(&self)->bool{
        let problem = self.U[0].clone();


        if let (Term::Function(f1),Term::Function(f2)) = (problem.0,problem.1){
            if f1.signature != f2.signature{
                return true;
            }
        }

        false
    }
}