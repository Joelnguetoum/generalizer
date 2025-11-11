use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::operators::Operator;
use crate::terms::function::Signature;
use crate::terms::term::Term;
use crate::utils::misc::{extract_inner, split_binary_op};

impl Term{


    pub fn to_interaction(&self)->Interaction{
        match &self{
            &Term::Variable(_)=> panic!("Cannot convert variable to interaction"),
            &Term::Function(f)=>{

                match (f.signature.arity,f.args.len()){
                    (0,0) => {
                        Interaction::parse_interaction(&f.signature.name)
                    },
                    (1,1) => {
                        match f.signature.name.as_str(){
                            "loopS"=>{
                                Interaction::wrap_loop(&f.args[0].to_interaction())
                            },
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    (2,2) => {
                        match f.signature.name.as_str(){
                            "seq"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Seq)
                            },
                            "par"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Par)
                            },
                            "alt"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Alt)
                            },
                            "tensor"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Tensor)
                            },
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    _ =>{
                        panic!("Invalid function arity during the conversion of a term into an interaction");
                    }
                }

            }
        }
    }



}



impl Interaction{
    pub fn parse_interaction(expr: &str) -> Interaction{
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

            if let (Interaction::Action(a1),Interaction::Action(a2)) = (Self::parse_interaction(&left),Self::parse_interaction(&right)){
                Interaction::Vp(a1.clone(),a2.clone())
            }
            else{
                panic!("Invalid Vp format: {}", expr);
            }

        } 
        else {
            panic!("Unsupported Interaction expression: {}", expr);
        }
    }
}