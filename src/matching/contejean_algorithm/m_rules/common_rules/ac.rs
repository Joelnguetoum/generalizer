use crate::matching::contejean_algorithm::error::MatchingError;
use crate::matching::contejean_algorithm::m_configuration::m_configuration::MConfiguration;
use crate::terms::function::{Function, FunctionSignature};
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

impl MConfiguration {

    pub fn is_variable_in_ps(&self, x: &Variable) -> bool {
        self.S.iter().any(|(y, _)| y == x) ||
            self.P.iter().any(|(x2, _, _, _)| x2 == x)
    }

    // Helper function to create AC term from arguments
    pub fn create_ac_term(signature: &FunctionSignature, args: Vec<Term>) -> Term {
        if args.len() == 1 {
            args[0].clone()
        } else {
            Term::Function(Function::new(signature, &args))
        }
    }

    pub fn can_apply_ac(&self) -> bool {
        if self.U.is_empty() {
            return false;
        }

        let problem = &self.U[0];

        if let (Term::Function(ref f1), Term::Function(ref f2)) = (&problem.0, &problem.1) {
            // Both must be AC with same signature
            if f1.signature == f2.signature
                && problem.0.is_head_function_associative_commutative()
                && f1.args.len() <= f2.args.len()
                && f1.args.len() > 1 {

                // First subterm of pattern must NOT be a variable and must match some subject subterm's head
                if !f1.args[0].is_variable() {
                    return f2.args.iter().any(|s_k| {
                        s_k.head_symbol_signature() == f1.args[0].head_symbol_signature()
                    });
                }
            }
        }
        false
    }




    pub fn ac(&self) -> Result<Vec<MConfiguration>, MatchingError> {
        let mut new_configs = Vec::new();
        let problem = self.U[0].clone();

        if let (Term::Function(f1), Term::Function(f2)) = (problem.0, problem.1) {
            let ac_sig = f1.signature.clone();

            // For each subject subterm that matches the head symbol of pattern's first subterm
            for (idx, s_k) in f2.args.iter().enumerate() {
                if s_k.head_symbol_signature() == f1.args[0].head_symbol_signature() {
                    let mut new_U = self.U.clone();
                    new_U.remove(0);

                    // Create new equations:
                    // g(p') = s_k
                    let first_eq = (f1.args[0].clone(), s_k.clone());

                    // p_2 + ... + p_m = s_1 + ... + s_{k-1} + s_{k+1} + ... + s_n
                    let mut remaining_pattern_args = f1.args[1..].to_vec();
                    let mut remaining_subject_args = f2.args.clone();
                    remaining_subject_args.remove(idx);

                    let remaining_pattern = Self::create_ac_term(&ac_sig, remaining_pattern_args);
                    let remaining_subject = Self::create_ac_term(&ac_sig, remaining_subject_args);

                    let second_eq = (remaining_pattern, remaining_subject);

                    new_U.insert(0, first_eq);
                    new_U.insert(1, second_eq);

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

        if new_configs.is_empty() {
            Err(MatchingError::MatchingFailure)
        } else {
            Ok(new_configs)
        }
    }

}