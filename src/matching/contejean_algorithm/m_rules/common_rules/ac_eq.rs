use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

impl MConfiguration {

    // AC= rule - variable in AC pattern with no current assignment (AC subject term)
    pub fn can_apply_ac_eq(&self) -> bool {
        if self.U.is_empty() {
            return false;
        }

        let problem = &self.U[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() < f2.args.len()
                && f1.args.len() >= 1 {

                if let Term::Variable(ref x) = &f1.args[0] {
                    // Variable must not be in P or S
                    return !self.is_variable_in_ps(x);
                }
            }
        }
        false
    }

    pub fn ac_eq(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let mut new_configs = Vec::new();
        let problem = self.U[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            if let Term::Variable(ref x) = &f1.args[0] {
                // Try each subject term
                for (idx, s_k) in f2.args.iter().enumerate() {
                    let mut new_U = self.U.clone();
                    new_U.remove(0);

                    // Create new existentially quantified variable
                    let new_y = Variable::fresh_variable(); // You might want a better naming scheme
                    let mut new_y_list = self.y.clone();
                    new_y_list.push(new_y.clone());

                    // Create: y + p_2 + ... + p_m = remaining subject
                    let mut new_pattern_args = vec![Term::Variable(new_y.clone())];
                    new_pattern_args.extend_from_slice(&f1.args[1..]);

                    let mut remaining_subject_args = f2.args.clone();
                    remaining_subject_args.remove(idx);

                    let new_pattern = Self::create_ac_term(&f1.signature, new_pattern_args);
                    let remaining_subject = Self::create_ac_term(&f1.signature, remaining_subject_args);

                    let pattern_eq = (new_pattern, remaining_subject);

                    // Add x = y + s_k to partial assignments
                    let mut new_P = self.P.clone();
                    new_P.push((x.clone(), f1.signature.clone(), new_y, s_k.clone()));

                    new_U.insert(0, pattern_eq);

                    let new_conf = MConfiguration::new(
                        new_y_list,
                        new_U,
                        new_P,
                        self.S.clone()
                    );
                    new_configs.push(new_conf);
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

