use std::collections::HashSet;
use crate::interactions::syntax::interaction::Interaction;

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub enum Operator {
    Alt,
    Par,
    Seq,
    Tensor,
    LoopS,
    Cons
}
impl Operator{
    pub fn to_string(&self)->String{
        match self.clone(){
            Operator::Seq => "seq".to_string(),
            Operator::Par => "par".to_string(),
            Operator::Alt => "alt".to_string(),
            Operator::LoopS => "loop".to_string(),
            Operator::Tensor => "tensor".to_string(),
            Operator::Cons => "cons".to_string(),
        }
    }
    pub fn string_operators(operators: &HashSet<Operator>)->String{
        let mut ret = String::from("");
        for op in operators {
            ret.push_str(&op.to_string());
            ret.push(' ');
        }

        ret
    }
}

impl Interaction{
    pub fn wrap_binary(int1: &Interaction, int2: &Interaction,op: &Operator) -> Interaction {
        match op {
            Operator::Seq =>{
                Interaction::Seq(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Par =>{
                Interaction::Par(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Alt =>{
                Interaction::Alt(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Tensor =>{
                Interaction::Tensor(Box::new(int1.clone()), Box::new(int2.clone()))
            }
            _ => panic!("Unsupported operator for wrap_binary"),
        }
    }

    pub fn wrap_loop(int: &Interaction)-> Interaction {
        Interaction::LoopS(Box::new(int.clone()))
    }
}



