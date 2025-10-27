use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    // AC_P≠ rule - variable in AC pattern with partial assignment (different AC symbol)
    pub fn can_apply_ac_p_diff(&self) -> bool {
        if self.U.is_empty() {
            return false;
        }

        let problem = &self.U[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() <= f2.args.len()
                && f1.args.len() >= 1 {

                if let Term::Variable(ref x) = &f1.args[0] {
                    for (x_prime, star_sig, _, s_prime) in &self.P {
                        if x == x_prime && &f1.signature != star_sig {
                            // We need to find s_k = s * s'_2 * ... * s'_n in subject
                            return f2.args.iter().any(|s_k| {
                                // Simplified check - in practice would need to decompose s_k
                                true // Non-deterministic choice
                            });
                        }
                    }
                }
            }
        }
        false
    }

    pub fn ac_p_diff(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let mut new_configs = Vec::new();
        let problem = self.U[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                for (x_prime, star_sig, y, s_prime) in &self.P {
                    if x == x_prime && &f1.signature != star_sig {
                        // Try each subject term as potential match
                        for (idx, s_k) in f2.args.iter().enumerate() {
                            // This is a simplified implementation
                            // In full version, we'd need to check if s_k can be decomposed as s * s'_2 * ... * s'_n

                            let mut new_U = self.U.clone();
                            new_U.remove(0);

                            // Create: y = s'_2 * ... * s'_n
                            // This is simplified - we'd need proper decomposition
                            let y_eq = (Term::Variable(y.clone()), s_k.clone()); // Simplified

                            // Create: p_2 + ... + p_m = remaining subject
                            let mut remaining_pattern_args = f1.args[1..].to_vec();
                            let mut remaining_subject_args = f2.args.clone();
                            remaining_subject_args.remove(idx);

                            let remaining_pattern = Self::create_ac_term(&f1.signature, remaining_pattern_args);
                            let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                            let pattern_eq = (remaining_pattern, remaining_subject);

                            new_U.insert(0, y_eq);
                            new_U.insert(1, pattern_eq);

                            let new_conf = MConfiguration::new(
                                self.y.clone(),
                                new_U,
                                self.P.clone(), // Keep the partial assignment
                                self.S.clone()
                            );
                            new_configs.push(new_conf);
                        }
                        break; // Only process first matching partial assignment
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