use std::collections::HashSet;
use crate::matching::contejean_algorithm::m_configuration::matching_process::MatchingProcess;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

impl MatchingProcess {

    pub fn contejean_algorithm(t1: &Term,t2: &Term) -> bool {
        let t1_can = t1.clone().to_canonical_form();
        let t2_can = t2.clone().to_canonical_form();
        let mut process = Self::init_process(&t1_can,&t2_can);

        process.match_ac()
    }
    pub fn match_ac(&mut self) -> bool {
        while let Some(config) = self.unsolved_configurations.pop() {

            self.process_m_configuration(config);
        }
        //println!("{}", self.solved_configurations.is_empty());
        !self.solved_configurations.is_empty()
    }

}



impl Term{
    pub fn to_canonical_form(&self) -> Term {
        match self {
            Term::Variable(v) => Term::Variable(v.clone()),
            Term::Function(f) => {
                // First, recursively convert all subterms to canonical form
                let canonical_args: Vec<Term> = f.args.iter()
                    .map(|arg| arg.to_canonical_form())
                    .collect();

                match f.signature.axioms.as_slice() {
                    // Free function symbol - just recursively canonicalize
                    [] => Term::Function(Function::new(&f.signature, &canonical_args)),

                    // Commutative but not associative - sort arguments
                    [Axioms::C] if !f.signature.axioms.contains(&Axioms::A) => {
                        let mut sorted_args = canonical_args;
                        sorted_args.sort_by(|a, b| a.canonical_compare(b));
                        Term::Function(Function::new(&f.signature, &sorted_args))
                    },

                    // Associative-commutative - flatten and sort
                    axioms if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C) => {
                        Self::flatten_and_sort_ac(&f.signature, &canonical_args)
                    },

                    // Other cases (associative only, etc.) - treat as free for now
                    _ => Term::Function(Function::new(&f.signature, &canonical_args)),
                }
            }
        }
    }

    /// Flattens AC function applications and sorts the arguments
    fn flatten_and_sort_ac(signature: &FunctionSignature, args: &[Term]) -> Term {
        let mut flattened = Vec::new();

        // Recursively flatten all arguments
        for arg in args {
            Self::flatten_ac_term(signature, arg, &mut flattened);
        }

        // Sort the flattened arguments using canonical ordering
        flattened.sort_by(|a, b| a.canonical_compare(b));

        // Reconstruct the term
        if flattened.len() == 1 {
            flattened[0].clone()
        } else {
            Term::Function(Function::new(signature, &flattened))
        }
    }

    /// Recursively flattens a term under an AC function symbol
    fn flatten_ac_term(signature: &FunctionSignature, term: &Term, result: &mut Vec<Term>) {
        match term {
            Term::Variable(_) => {
                result.push(term.clone());
            },
            Term::Function(f) => {
                if f.signature == *signature {
                    // Same AC symbol - recursively flatten all arguments
                    for arg in &f.args {
                        Self::flatten_ac_term(signature, arg, result);
                    }
                } else {
                    // Different symbol - keep as is
                    result.push(term.clone());
                }
            }
        }
    }

    // Comparison function for canonical ordering of terms
    /// This defines a total order on canonical forms
    pub fn canonical_compare(&self, other: &Term) -> std::cmp::Ordering {
        match (self, other) {
            // Variables come before functions
            (Term::Variable(_), Term::Function(_)) => std::cmp::Ordering::Less,
            (Term::Function(_), Term::Variable(_)) => std::cmp::Ordering::Greater,

            // Compare variables by name
            (Term::Variable(v1), Term::Variable(v2)) => v1.name().cmp(&v2.name()),

            // Compare functions
            (Term::Function(f1), Term::Function(f2)) => {
                // First compare by function symbol
                let symbol_cmp = f1.signature.name.cmp(&f2.signature.name);
                if symbol_cmp != std::cmp::Ordering::Equal {
                    return symbol_cmp;
                }

                // Then compare by arity
                let arity_cmp = f1.args.len().cmp(&f2.args.len());
                if arity_cmp != std::cmp::Ordering::Equal {
                    return arity_cmp;
                }

                // Finally compare arguments lexicographically
                for (a1, a2) in f1.args.iter().zip(f2.args.iter()) {
                    let arg_cmp = a1.canonical_compare(a2);
                    if arg_cmp != std::cmp::Ordering::Equal {
                        return arg_cmp;
                    }
                }

                std::cmp::Ordering::Equal
            }
        }
    }

    /// Checks if two terms are equal modulo AC by comparing their canonical forms
    pub fn equals_modulo_ac(&self, other: &Term) -> bool {
        let canonical_self = self.to_canonical_form();
        let canonical_other = other.to_canonical_form();
        canonical_self == canonical_other
    }

    /// Returns the set of free variables in the term
    pub fn free_variables(&self) -> HashSet<Variable> {
        match self {
            Term::Variable(v) => {
                let mut set = HashSet::new();
                set.insert(v.clone());
                set
            },
            Term::Function(f) => {
                f.args.iter()
                    .flat_map(|arg| arg.free_variables())
                    .collect()
            }
        }
    }

    /// Checks if the term is in canonical form
    pub fn is_canonical(&self) -> bool {
        match self {
            Term::Variable(_) => true,
            Term::Function(f) => {
                // Check if all subterms are canonical
                if !f.args.iter().all(|arg| arg.is_canonical()) {
                    return false;
                }

                match f.signature.axioms.as_slice() {
                    // Free function - any order is fine
                    [] => true,

                    // Commutative - arguments must be sorted
                    [Axioms::C] if !f.signature.axioms.contains(&Axioms::A) => {
                        Self::is_sorted_by(&f.args, |a, b| a.canonical_compare(b) != std::cmp::Ordering::Greater)
                    },

                    // AC - must be flattened and sorted
                    axioms if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C) => {
                        // Check if sorted
                        let sorted = Self::is_sorted_by(&f.args, |a, b| a.canonical_compare(b) != std::cmp::Ordering::Greater);

                        // Check if flattened (no nested same AC symbols)
                        let flattened = f.args.iter().all(|arg| {
                            if let Term::Function(child_f) = arg {
                                child_f.signature != f.signature
                            } else {
                                true
                            }
                        });

                        sorted && flattened
                    },

                    // Other cases
                    _ => true,
                }
            }
        }
    }

    /// Helper function to check if a slice is sorted according to a comparison function
    fn is_sorted_by<T, F>(slice: &[T], mut compare: F) -> bool
    where
        F: FnMut(&T, &T) -> bool
    {
        slice.windows(2).all(|w| compare(&w[0], &w[1]))
    }
}