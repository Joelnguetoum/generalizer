use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::term::Term;

impl MConfiguration {

    // AC_P= rule - variable in AC pattern with partial assignment (same AC symbol)
    pub fn can_apply_ac_p_eq(&self) -> bool {
        if self.u.is_empty() {
            return false;
        }

        let problem = &self.u[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() < f2.args.len()
                && f1.args.len() >= 1 {

                if let Term::Variable(ref x) = &f1.args[0] {
                    return self.p.iter().any(|(x_prime, plus_sig, _, s_prime)| {
                        x == x_prime && plus_sig == &f1.signature && f2.args.contains(s_prime)
                    });
                }
            }
        }
        false
    }

    pub fn ac_p_eq(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let problem = self.u[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                for (x_prime, plus_sig, y, s_prime) in &self.p {
                    if x == x_prime && plus_sig == &f1.signature && f2.args.contains(s_prime) {
                        if let Some(idx) = f2.args.iter().position(|arg| arg == s_prime) {
                            let mut new_u = self.u.clone();
                            new_u.remove(0);

                            // Create: y + p_2 + ... + p_m = remaining subject
                            let mut new_pattern_args = vec![Term::Variable(y.clone())];
                            new_pattern_args.extend_from_slice(&f1.args[1..]);

                            let mut remaining_subject_args = f2.args.clone();
                            remaining_subject_args.remove(idx);

                            let new_pattern = Self::create_ac_term(&f1.signature, new_pattern_args);
                            let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                            new_u.insert(0, (new_pattern, remaining_subject));

                            let new_conf = MConfiguration::new(
                                self.y.clone(),
                                new_u,
                                self.p.clone(), // Keep partial assignment
                                self.s.clone()
                            );
                            return Ok(vec![new_conf]);
                        }
                    }
                }
            }
        }

        Err(MatchingError::InvalidRuleApplication)
    }


}