
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags, special_fold_recursive_tensor_frags};
use crate::maude::from_maude::utils::{extract_inner, split_operands};
use crate::utils::misc::split_binary_op;

// Parses Maude interaction expressions into `Interaction`.
pub fn parse_maude_interaction(expr: &str,order:bool) -> Interaction {
    let expr = expr.trim();

    if expr == "Empty" {
        Interaction::Empty
    }
    else if expr.starts_with("Action(") {
        /*
        let parts: Vec<&str> = extract_inner(expr, "Action")
            .split(',')
            .map(|s| s.trim())
            .collect();

         */
        let p0= extract_inner(expr, "Action");
        let p1 = p0.split(',');
        let p2 = p1.map(|s| s.trim()).filter(|s| !s.is_empty());
        let parts: Vec<&str> = p2.collect();
       // println!("problematic parts: {:?}", parts);
        if parts.len() != 4 {
            panic!("Invalid Action format: {}", expr);
        }

        let ty = if parts[2].to_string().eq(&"0".to_string()) {
            ActionType::Emission
        }
        else{
            ActionType::Reception
        };
        let gate_id = if parts[3].to_string().eq(&"0".to_string()){
            None
        }
        else{
            Some(parts[3].parse::<usize>().ok().unwrap())
        };

        let act = Action::new_with_id_raw(parts[0].parse().unwrap(),parts[1].parse().unwrap(),ty,gate_id);
        Interaction::Action(act)

    } else if expr.starts_with("Vp(") {
        let (left, right) = split_binary_op(expr, "Vp");

        if let (Interaction::Action(a1),Interaction::Action(a2)) = (parse_maude_interaction(&left,order),parse_maude_interaction(&right,order)){
            Interaction::Vp(a1.clone(),a2.clone())
        }
        else{
            panic!("Invalid Vp format: {}", expr);
        }

    } else if expr.starts_with("seq(") {

        let inner = extract_inner(expr, "seq");
        let operands = split_operands(&inner);
        let mut parsed_operands = parse_operands_int(&operands,order);
        special_fold_recursive_seq_frags(&mut parsed_operands)

    } else if expr.starts_with("alt(") {
        let inner = extract_inner(expr, "alt");
        let operands = split_operands(&inner);
        let mut parsed_operands = parse_operands_int(&operands,order);

        if order{
            parsed_operands.sort_by(|a, b| a.gates_cmp(b));
        }
        special_fold_recursive_alt_frags(&mut parsed_operands)

    } else if expr.starts_with("par(") {
        let inner = extract_inner(expr, "par");
        let operands = split_operands(&inner);
        let mut parsed_operands = parse_operands_int(&operands,order);

        if order{
            parsed_operands.sort_by(|a, b| a.gates_cmp(b));
        }

        special_fold_recursive_par_frags(&mut parsed_operands)
    } else if expr.starts_with("tensor(") {
        let inner = extract_inner(expr, "tensor");
        let operands = split_operands(&inner);
        let mut parsed_operands = parse_operands_int(&operands,order);

        if order{
            parsed_operands.sort_by(|a, b| a.gates_cmp(b));
        }

        special_fold_recursive_tensor_frags(&mut parsed_operands)
    } else if expr.starts_with("loopS(") {
        let inner = extract_inner(expr, "loopS");
        Interaction::LoopS(Box::new(parse_maude_interaction(&inner,order)))
    } else {
        panic!("Unsupported Interaction expression: {}", expr);
    }
}

pub fn parse_operands_int(operands: &Vec<String>,order:bool) -> Vec<Interaction> {
    let mut results = Vec::new();

    for operant in operands {
        results.push(parse_maude_interaction(operant,order));
    }
    results
}

