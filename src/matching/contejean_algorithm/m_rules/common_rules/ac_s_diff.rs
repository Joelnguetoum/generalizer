use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    // AC_S≠ rule - variable in AC pattern with existing solved value (non-AC top)
    pub fn can_apply_ac_s_diff(&self) -> bool {
        if self.u.is_empty() {
            return false;
        }

        let problem = &self.u[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() <= f2.args.len()
                && f1.args.len() >= 1 {

                // First subterm must be variable
                if let Term::Variable(ref x) = &f1.args[0] {
                    // Variable must be in solved part with non-AC top symbol
                    for (x_prime, s_val) in &self.s {
                        if x == x_prime && !s_val.is_head_function_associative_commutative() {
                            // s_val must appear in subject args
                            return f2.args.contains(s_val);
                        }
                    }
                }
            }
        }
        false
    }

    pub fn ac_s_diff(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let problem = self.u[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                for (x_prime, s_val) in &self.s {
                    if x == x_prime && f2.args.contains(s_val) {
                        let mut new_u = self.u.clone();
                        new_u.remove(0);

                        // Find index of s_val in subject
                        let idx = f2.args.iter().position(|arg| arg == s_val).unwrap();

                        // Create remaining pattern: p_2 + ... + p_m
                        let mut remaining_pattern_args = f1.args[1..].to_vec();
                        let remaining_pattern = Self::create_ac_term(&f1.signature, remaining_pattern_args);

                        // Create remaining subject: s_1 + ... + s_{k-1} + s_{k+1} + ... + s_n
                        let mut remaining_subject_args = f2.args.clone();
                        remaining_subject_args.remove(idx);
                        let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                        new_u.insert(0, (remaining_pattern, remaining_subject));

                        let new_conf = MConfiguration::new(
                            self.y.clone(),
                            new_u,
                            self.p.clone(),
                            self.s.clone()
                        );
                        return Ok(vec![new_conf]);
                    }
                }
            }
        }

        Err(MatchingError::InvalidRuleApplication)
    }

}