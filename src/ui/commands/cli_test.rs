use clap::ArgMatches;
use crate::matching::brute_force::matching_ac::brute_force_match_modulo_ac;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;

pub fn cli_test(matches: &ArgMatches) {

    let sig_f = FunctionSignature::new("f".to_string(),2,vec![Axioms::A]);
    let sig_a = FunctionSignature::new("a".to_string(),0,vec![]);
    let sig_b = FunctionSignature::new("b".to_string(),0,vec![]);
    let sig_c = FunctionSignature::new("c".to_string(),0,vec![]);
    let sig_d = FunctionSignature::new("d".to_string(),0,vec![]);

    let a = Term::Function(Function::new(&sig_a,&vec![]));
    let b = Term::Function(Function::new(&sig_b,&vec![]));
    let c = Term::Function(Function::new(&sig_c,&vec![]));
    let d = Term::Function(Function::new(&sig_d,&vec![]));

    let t1 = Term::Function(Function::new(&sig_f,&vec![a.clone(),b.clone()]));
    let t2 = Term::Function(Function::new(&sig_f,&vec![b.clone(),c.clone()]));

    let t3 = Term::Function(Function::new(&sig_f,&vec![a.clone(),t2.clone()]));
    let t4 = Term::Function(Function::new(&sig_f,&vec![t1.clone(),c.clone()]));

    let x1 = Term::Variable(Variable::fresh_variable());
    let x2 = Term::Variable(Variable::fresh_variable());
    let x3 = Term::Variable(Variable::fresh_variable());
    let x4 = Term::Variable(Variable::fresh_variable());

    let t4 = Term::Function(Function::new(&sig_f,&vec![x1.clone(),x2.clone()]));

    let t5 = Term::Function(Function::new(&sig_f,&vec![c.clone(),x3.clone()]));

    let t6 = Term::Function(Function::new(&sig_f,&vec![b.clone(),t5.clone()]));

    let t7 = Term::Function(Function::new(&sig_f,&vec![a.clone(),t6.clone()]));

    let verdict = brute_force_match_modulo_ac(&t4, &t7);

    println!("{}", verdict);


}