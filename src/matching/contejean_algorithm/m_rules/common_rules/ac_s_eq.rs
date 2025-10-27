use std::collections::HashSet;
use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    pub fn can_apply_ac_s_eq(&self) -> bool {
        if self.U.is_empty() {
            return false;
        }

        let problem = &self.U[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() >= 1 {

                if let Term::Variable(ref x) = &f1.args[0] {
                    for (x_prime, s_val) in &self.S {
                        if x == x_prime && s_val.is_head_function_associative_commutative() {
                            if let Term::Function(ref s_func) = s_val {
                                return s_func.signature == f1.signature;
                            }
                        }
                    }
                }
            }
        }
        false
    }


    pub fn ac_s_eq(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let mut new_configs = Vec::new();
        let problem = self.U[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                for (x_prime, s_val) in &self.S {
                    if x == x_prime {
                        if let Term::Function(ref s_func) = s_val {
                            // This is complex - we need to find how to distribute subject args
                            // between the solved AC term and remaining pattern
                            // For now, implement a simplified version

                            let subject_args_set: HashSet<Term> = f2.args.iter().cloned().collect();
                            let solved_args_set: HashSet<Term> = s_func.args.iter().cloned().collect();

                            // Check if solved args are subset of subject args
                            if solved_args_set.is_subset(&subject_args_set) {
                                let mut remaining_subject_args = f2.args.clone();

                                // Remove all solved args from subject
                                for solved_arg in &s_func.args {
                                    if let Some(pos) = remaining_subject_args.iter().position(|a| a == solved_arg) {
                                        remaining_subject_args.remove(pos);
                                    }
                                }

                                let pattern_remaining_count = f1.args.len() - 1;

                                // Check if remaining subject can match remaining pattern
                                if remaining_subject_args.len() >= pattern_remaining_count {
                                    let mut new_U = self.U.clone();
                                    new_U.remove(0);

                                    // Create remaining pattern
                                    let mut remaining_pattern_args = f1.args[1..].to_vec();
                                    let remaining_pattern = Self::create_ac_term(&f1.signature, remaining_pattern_args);

                                    // Create remaining subject
                                    let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                                    new_U.insert(0, (remaining_pattern, remaining_subject));

                                    let new_conf = MConfiguration::new(
                                        self.y.clone(),
                                        new_U,
                                        self.P.clone(),
                                        self.S.clone()
                                    );
                                    new_configs.push(new_conf);
                                }
                            }
                        }
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