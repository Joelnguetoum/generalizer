use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_rules::m_rule::MRule;
use crate::terms::function::FunctionSignature;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;
#[derive(Clone, Debug)]
pub struct MConfiguration {
    pub y: Vec<Variable>,
    pub U: Vec<(Term,Term)>,
    pub P: Vec<(Variable,FunctionSignature,Variable,Term)>,
    pub S: Vec<(Variable,Term)>
}

impl MConfiguration {
    pub fn new(y: Vec<Variable>, U: Vec<(Term,Term)>, P: Vec<(Variable,FunctionSignature,Variable,Term)>, S: Vec<(Variable,Term)> ) -> MConfiguration {
        MConfiguration{ y, U, P, S }
    }

    pub fn init_m_conf(t1: &Term, t2: &Term) -> MConfiguration {
        let y = vec![];
        let U = vec![(t1.clone(),t2.clone())];
        let P = vec![];
        let S = vec![];

        MConfiguration::new(y, U, P, S)
    }
    pub fn applicable_rule(&self) -> Option<MRule> {
        if self.U.is_empty() {
            return None;
        }

        if self.can_apply_dec(){
            Some(MRule::Dec)
        } else if self.can_apply_dec_c(){
            Some(MRule::DecC)
        } else if self.can_apply_clash(){
            Some(MRule::Clash)
        } else if self.can_apply_clash_s(){
            Some(MRule::ClashS)
        }else if self.can_apply_clash_p(){
            Some(MRule::ClashP)
        }
        else if self.can_apply_merge_s(){
            Some(MRule::MergeS)
        }else if self.can_apply_merge_p(){
            Some(MRule::MergeP)
        }else if self.can_apply_solve(){
            Some(MRule::Solve)
        }else if self.can_apply_ac(){
            Some(MRule::Ac)
        }else if self.can_apply_ac_diff(){
            Some(MRule::AcDiff)
        }else if self.can_apply_ac_eq(){
            Some(MRule::AcEq)
        }else if self.can_apply_ac_p_diff(){
            Some(MRule::AcPDiff)
        }else if self.can_apply_ac_p_eq(){
            Some(MRule::AcPEq)
        }else if self.can_apply_ac_s_diff(){
            Some(MRule::AcSDiff)
        }else if self.can_apply_ac_s_eq(){
            Some(MRule::AcSEq)
        }
        else {
            None
        }

    }

    pub fn apply_rule(&self, rule: MRule) -> Result<Vec<MConfiguration>, MatchingError> {
        match rule {
            MRule::Ac => self.ac(),
            MRule::AcDiff => self.ac_diff(),
            MRule::AcEq => self.ac_eq(),
            MRule::AcPDiff=> self.ac_p_diff(),
            MRule::AcPEq => self.ac_p_eq(),
            MRule::AcSDiff=> self.ac_s_diff(),
            MRule::AcSEq=> self.ac_s_eq(),
            MRule::Clash => Err(MatchingError::MatchingFailure),
            MRule::ClashS => Err(MatchingError::MatchingFailure),
            MRule::ClashP => Err(MatchingError::MatchingFailure),
            MRule::MergeS => self.merge_s(),
            MRule::MergeP => self.merge_p(),
            MRule::Solve => self.solve(),
            MRule::Dec => self.dec(),
            MRule::DecC => self.dec_c(),
            _ =>{
                Err(MatchingError::UnknownRule)
            }
        }
    }
}