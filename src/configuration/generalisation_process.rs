use std::collections::{HashMap, HashSet};
use crate::configuration::configuration::Configuration;
use crate::generaliser::generaliser::Generaliser;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;

#[derive(Debug)]
pub struct GeneralisationProcess {
    pub solved_configurations: Vec<Configuration>,
    pub unsolved_configurations: Vec<Configuration>
}

impl GeneralisationProcess {
    pub fn new(conf: &Configuration) -> Self {
        Self{solved_configurations: Vec::new(),unsolved_configurations:vec![conf.clone()]}
    }

    pub fn init_process(t1: &Term, t2: &Term) -> GeneralisationProcess {
        let conf = Configuration::init_conf(t1,t2);

        GeneralisationProcess::new(&conf)
    }

    pub fn to_generalisers(&self) -> Vec<Generaliser> {
        let generalisers: Vec<Generaliser> = self
            .solved_configurations
            .iter()
            .map(|conf| {
                let mut g = conf.to_generaliser();
                // Normalize + unflatten
                g.t = g.t.normalize_mod_ac().unflatten_mod_ac();
                g
            })
            .collect();

        let mut filtered = Vec::new();

        'outer: for gen in &generalisers {
            // 1️⃣ Skip if more general one exists (mod AC)
            for other in &generalisers {
                if std::ptr::eq(gen, other) {
                    continue;
                }
                if gen.t.is_more_general_mod_ac(&other.t)
                    && !gen.t.is_alpha_equivalent_mod_ac(&other.t)
                {
                    continue 'outer;
                }
            }

            // 2️⃣ Skip if α-equivalent modulo AC
            if filtered
                .iter()
                .any(|existing: &Generaliser| gen.t.is_alpha_equivalent_mod_ac(&existing.t))
            {
                continue 'outer;
            }

            // 3️⃣ Otherwise keep
            filtered.push(gen.clone());
        }

        filtered
    }



}



impl Term{

    pub fn is_more_general_mod_ac(&self, other: &Term) -> bool {
        // Full AC flattening before matching
        let s = self.normalize_mod_ac_flat();
        let o = other.normalize_mod_ac_flat();

        Self::ac_match_flat(&s, &o, &mut std::collections::HashMap::new())
    }
    pub fn normalize_mod_ac_flat(&self) -> Term {
        match self {
            Term::Variable(_) => self.clone(),
            Term::Function(f) => {
                let sig = f.signature.clone();
                let is_a = sig.axioms.contains(&Axioms::A);
                let is_c = sig.axioms.contains(&Axioms::C);

                let mut flat_args: Vec<Term> = Vec::new();

                for arg in &f.args {
                    let norm = arg.normalize_mod_ac_flat();
                    if let Term::Function(subf) = &norm {
                        if subf.signature.name == sig.name && (is_a || is_c) {
                            // Flatten nested occurrences
                            flat_args.extend(subf.args.clone());
                        } else {
                            flat_args.push(norm);
                        }
                    } else {
                        flat_args.push(norm);
                    }
                }

                if is_c {
                    flat_args.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                }

                Term::Function(Function { signature: sig, args: flat_args })
            }
        }
    }
    fn ac_match_flat(
        t1: &Term,
        t2: &Term,
        subst: &mut std::collections::HashMap<String, Term>,
    ) -> bool {
        match (t1, t2) {
            (Term::Variable(v), _) => {
                let name = format!("{:?}", v);
                if let Some(bound) = subst.get(&name) {
                    bound == t2
                } else {
                    subst.insert(name, t2.clone());
                    true
                }
            }

            (Term::Function(f1), Term::Function(f2)) => {
                if f1.signature.name != f2.signature.name {
                    return false;
                }

                let is_a = f1.signature.axioms.contains(&Axioms::A);
                let is_c = f1.signature.axioms.contains(&Axioms::C);

                if !(is_a || is_c) {
                    // Non-AC: normal structural matching
                    if f1.args.len() != f2.args.len() {
                        return false;
                    }
                    for (a1, a2) in f1.args.iter().zip(&f2.args) {
                        if !Self::ac_match_flat(a1, a2, subst) {
                            return false;
                        }
                    }
                    return true;
                }

                // --- AC case ---
                let mut args1 = f1.args.clone();
                let mut args2 = f2.args.clone();

                // Already flattened by normalize_mod_ac_flat()

                // Greedy matching: variables in args1 can absorb one or several args from args2
                if args1.len() > args2.len() {
                    return false;
                }

                let mut i2 = 0;
                for (i1, a1) in args1.iter().enumerate() {
                    if i1 == args1.len() - 1 {
                        // last var absorbs remainder
                        let remainder = args2[i2..].to_vec();
                        let term = if remainder.len() == 1 {
                            remainder[0].clone()
                        } else {
                            Term::Function(Function {
                                signature: f1.signature.clone(),
                                args: remainder,
                            })
                        };
                        return Self::ac_match_flat(a1, &term, subst);
                    }

                    if i2 >= args2.len() {
                        return false;
                    }

                    if !Self::ac_match_flat(a1, &args2[i2], subst) {
                        return false;
                    }
                    i2 += 1;
                }
                true
            }

            _ => false,
        }
    }


    //////////////////////////////////////////
    /////////////////////////////////////////
    pub fn is_alpha_equivalent_mod_ac(&self, other: &Term) -> bool {
        let t1 = self.normalize_mod_ac();
        let t2 = other.normalize_mod_ac();
        Self::alpha_eq_with_map(&t1, &t2, &mut HashMap::new(), &mut HashMap::new())
    }

    fn alpha_eq_with_map(
        t1: &Term,
        t2: &Term,
        map1: &mut HashMap<String, String>,
        map2: &mut HashMap<String, String>,
    ) -> bool {
        match (t1, t2) {
            // Two variables: check consistent renaming
            (Term::Variable(v1), Term::Variable(v2)) => {
                let key1 = format!("{:?}", v1);
                let key2 = format!("{:?}", v2);
                match (map1.get(&key1), map2.get(&key2)) {
                    (Some(m1), Some(m2)) => m1 == &key2 && m2 == &key1,
                    (Some(m1), None) => m1 == &key2,
                    (None, Some(m2)) => m2 == &key1,
                    (None, None) => {
                        map1.insert(key1.clone(), key2.clone());
                        map2.insert(key2.clone(), key1.clone());
                        true
                    }
                }
            }

            // Two functions: same head symbol, same arity, recursively check args
            (Term::Function(f1), Term::Function(f2)) => {
                if f1.signature.name != f2.signature.name {
                    return false;
                }
                if f1.args.len() != f2.args.len() {
                    return false;
                }

                for (a1, a2) in f1.args.iter().zip(&f2.args) {
                    if !Self::alpha_eq_with_map(a1, a2, map1, map2) {
                        return false;
                    }
                }
                true
            }

            _ => false,
        }
    }



    pub fn unflatten_mod_ac(&self) -> Term {
        match self {
            Term::Variable(_) => self.clone(),

            Term::Function(f) => {
                let sig = f.signature.clone();
                let arity = sig.arity;

                // First unflatten all subterms
                let mut args: Vec<Term> = f.args.iter().map(|t| t.unflatten_mod_ac()).collect();

                // If this symbol is associative (possibly also commutative)
                let is_a = f.signature.axioms.contains(&Axioms::A);
                let is_c = f.signature.axioms.contains(&Axioms::C);

                if is_a || is_c {
                    // Reconstruct a nested structure of the right arity
                    return Self::rebuild_assoc_func(sig, &mut args, arity);
                }

                // Non-associative function: keep as is (already arity-correct)
                Term::Function(Function { signature: sig, args })
            }
        }
    }


    fn rebuild_assoc_func(sig: FunctionSignature, args: &mut Vec<Term>, arity: usize) -> Term {
        // Base case: if the number of args <= arity, we’re done
        if args.len() <= arity {
            return Term::Function(Function {
                signature: sig.clone(),
                args: args.clone(),
            });
        }

        // Otherwise, build nested applications:
        // e.g. for arity = 2, args = [a, b, c, d]
        // we build f(a, f(b, f(c, d)))
        let mut it = args.iter().cloned().rev();
        let mut acc = it.next().unwrap();

        while let Some(a) = it.next() {
            acc = Term::Function(Function {
                signature: sig.clone(),
                args: vec![a, acc],
            });
        }
        acc
    }
}