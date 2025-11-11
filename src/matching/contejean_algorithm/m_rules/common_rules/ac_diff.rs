use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    // AC≠ rule - variable in AC pattern with no current assignment
    pub fn can_apply_ac_diff(&self) -> bool {
        if self.u.is_empty() {
            return false;
        }

        let problem = &self.u[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() <= f2.args.len()
                && f1.args.len() >= 1 {

                if let Term::Variable(ref x) = &f1.args[0] {
                    // Variable must not be in P or S, and subject terms must not have AC top
                    return !self.is_variable_in_ps(x) &&
                        f2.args.iter().any(|s_k| !s_k.is_head_function_associative_commutative());
                }
            }
        }
        false
    }

    pub fn ac_diff(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let mut new_configs = Vec::new();
        let problem = self.u[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                // Try each non-AC subject term
                for (idx, s_k) in f2.args.iter().enumerate() {
                    if !s_k.is_head_function_associative_commutative() {
                        let mut new_u = self.u.clone();
                        new_u.remove(0);

                        // Create: p_2 + ... + p_m = remaining subject
                        let mut remaining_pattern_args = f1.args[1..].to_vec();
                        let mut remaining_subject_args = f2.args.clone();
                        remaining_subject_args.remove(idx);

                        let remaining_pattern = Self::create_ac_term(&f1.signature, remaining_pattern_args);
                        let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                        let pattern_eq = (remaining_pattern, remaining_subject);

                        // Add x = s_k to solved part
                        let mut new_s = self.s.clone();
                        new_s.push((x.clone(), s_k.clone()));

                        new_u.insert(0, pattern_eq);

                        let new_conf = MConfiguration::new(
                            self.y.clone(),
                            new_u,
                            self.p.clone(),
                            new_s
                        );
                        new_configs.push(new_conf);
                    }
                }
            }
        }

        if new_configs.is_empty() {
            Err(MatchingError::MatchingFailure)
        } else {
            Ok(new_configs)
        }
    }

}