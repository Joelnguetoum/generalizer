



use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;
use crate::utils::combinatorics::Combinatorics;

impl Configuration {
    pub fn can_apply_decompose_ac(&self) -> bool {
        let aut = self.active[0].clone();

        if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
            && aut.t1.is_head_function_associative_commutative()
         {
            return true;
        }
        false
    }
    pub fn decompose_ac(&self) -> Result<Vec<Configuration>, ConfigurationError> {
       // println!("decompose_ac");

        let mut return_configurations = Vec::new();

        let mut active = self.active.clone();

        //Since this is decompose, we remove the concerned AUT
        let aut = active.remove(0);

        match (aut.t1.clone().assoc_flatten(), aut.t2.clone().assoc_flatten()) {
            (Term::Function(f1), Term::Function(f2)) => {

                for (t1,t1_prime,t2,t2_prime) in Self::assoc_comm_dec_groups(&f1.signature,&f1.args, &f2.args) {
                    let mut new_active = active.clone();
                    let mut new_store = self.store.clone();
                    let mut new_sub = self.sub.clone();



                    /*
                    let x1 = Variable::fresh_variable();
                    let x2 = Variable::fresh_variable();

                    let aut1 = AUT::new(x1.clone(), t1.clone(), t1_prime.clone());
                    let aut2 = AUT::new(x2.clone(), t2.clone(), t2_prime.clone());

                    sub_term_args.push(Term::Variable(x1.clone()));
                    sub_term_args.push(Term::Variable(x2.clone()));
                     */

                    let (x1, x2, aut1, aut2) = self.with_fresh_aut_pair(
                        t1, t1_prime, t2, t2_prime
                    );

                    new_active.insert(0,aut1.clone());
                    new_active.insert(0,aut2.clone());

                    let mut sub = Substitution::new();
                    let sub_term_args = vec![Term::Variable(x1), Term::Variable(x2)];

                    sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
                    new_sub.push(sub);

                    //let conf = Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose_AC"));

                    let conf = self.create_new_config(new_active, new_store, new_sub, &Rule::DecomposeAC);
                    return_configurations.push(conf);
                }

                return Ok(return_configurations);
            },
            _ => Err(ConfigurationError::NonGroundTerm)
        }
    }

    pub fn assoc_comm_dec_groups(
        sig: &FunctionSignature,
        args1: &Vec<Term>,
        args2: &Vec<Term>,
    ) -> Vec<(Term, Term, Term, Term)> {
        let mut quadruples = Vec::new();

        // Left decompositions
        for j in 0..args2.len() {
            let n = args1.len();
            for subset_size in 1..=n - 1 {
                let indices: Vec<usize> = (0..n).collect();
                let combos = Combinatorics::combinations(&indices, subset_size);

                for combo in combos {
                    let (vec1, vec2) = Combinatorics::split_vector(args1, &combo);
                    let (vec3, vec4) = Combinatorics::split_vector(args2, &[j]);

                    let t1 = Term::assoc_wrap(sig, &vec1);
                    let t1_prime = Term::assoc_wrap(sig, &vec3);
                    let t2 = Term::assoc_wrap(sig, &vec2);
                    let t2_prime = Term::assoc_wrap(sig, &vec4);

                    quadruples.push((t1, t1_prime, t2, t2_prime));
                }
            }
        }

        // Right decompositions
        for i in 0..args1.len() {
            let n = args2.len();
            for subset_size in 1..=n - 1 {
                let indices: Vec<usize> = (0..n).collect();
                let combos = Combinatorics::combinations(&indices, subset_size);

                for combo in combos {
                    let (vec1, vec2) = Combinatorics::split_vector(args1, &[i]);
                    let (vec3, vec4) = Combinatorics::split_vector(args2, &combo);

                    let t1 = Term::assoc_wrap(sig, &vec1);
                    let t1_prime = Term::assoc_wrap(sig, &vec3);
                    let t2 = Term::assoc_wrap(sig, &vec2);
                    let t2_prime = Term::assoc_wrap(sig, &vec4);

                    quadruples.push((t1, t1_prime, t2, t2_prime));
                }
            }
        }

        quadruples
    }


    /*
    pub fn assoc_comm_dec_groups(sig: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
        let mut quadruples = Vec::new();

        //Left
        for j in 0..args2.len() {
            let n = args1.len();
            for subset_size in 1..=n-1 {
                let indices: Vec<usize> = (0..n).collect();
                let combos = Self::combinations(&indices, subset_size);

                for combo in combos {
                    let (vec1,vec2) = Self::split_vector(&args1,&combo);

                    let (vec3,vec4) = Self::split_vector(&args2,&Vec::from(&[j]));

                    let t1 = Term::assoc_wrap(sig,&vec1);
                    let t1_prime = Term::assoc_wrap(sig,&vec3);

                    let t2 = Term::assoc_wrap(sig,&vec2);
                    let t2_prime = Term::assoc_wrap(sig,&vec4);

                    quadruples.push((t1,t1_prime,t2,t2_prime));
                }



            }



        }

        //Right
        for i in 0..args1.len() {
            let n = args2.len();
            for subset_size in 1..=n-1 {
                let indices: Vec<usize> = (0..n).collect();
                let combos = Self::combinations(&indices, subset_size);

                for combo in combos {

                    let (vec1,vec2) = Self::split_vector(&args1,&Vec::from(&[i]));

                    let (vec3,vec4) = Self::split_vector(&args2,&combo);



                    let t1 = Term::assoc_wrap(sig,&vec1);
                    let t1_prime = Term::assoc_wrap(sig,&vec3);

                    let t2 = Term::assoc_wrap(sig,&vec2);
                    let t2_prime = Term::assoc_wrap(sig,&vec4);

                    quadruples.push((t1,t1_prime,t2,t2_prime));
                }



            }



        }



        quadruples
    }

    pub fn split_vector(
        base_vector: &Vec<Term>,
        index_vec: &Vec<usize>,
    ) -> (Vec<Term>, Vec<Term>) {

        // Put indices into a set for O(1) lookup
        //let index_set: HashSet<usize> = HashSet::from(index_vec.clone());

        let mut selected = Vec::new();
        let mut remaining = Vec::new();

        for (i, item) in base_vector.iter().enumerate() {
            if index_vec.contains(&i) {
                selected.push(item.clone());
            } else {
                remaining.push(item.clone());
            }
        }

        (selected, remaining)
    }

    fn combinations<T: Clone>(arr: &[T], k: usize) -> Vec<Vec<T>> {
        let mut result = Vec::new();
        let mut combo = Vec::new();
        Self::generate_combinations(arr, k, 0, &mut combo, &mut result);
        result
    }


    fn generate_combinations<T: Clone>(
        arr: &[T],
        k: usize,
        start: usize,
        combo: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if combo.len() == k {
            result.push(combo.clone());
            return;
        }


        for i in start..arr.len() {
            combo.push(arr[i].clone());
            Self::generate_combinations(arr, k, i + 1, combo, result);
            combo.pop();
        }
    }

     */

}



