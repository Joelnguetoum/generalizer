
use crate::interactions::syntax::action::ActionType;
use crate::interactions::syntax::interaction::Interaction;

pub fn to_maude_interaction(i: &Interaction) -> String {
    match i {
        Interaction::Empty => "Empty".to_string(),
        Interaction::Action(a) =>{
            let ty = if a.action_type==ActionType::Emission{
                0
            }
            else{
                1
            };

            let gate_id = a.gate_id.unwrap_or_else(|| 0);

            format!("Action({},{},{},{})",a.lf_id,a.ms_id,ty,gate_id)
        },
        Interaction::Vp(left, right) =>{
            format!("Vp({}, {})", to_maude_interaction(&Interaction::Action(left.clone())), to_maude_interaction(&Interaction::Action(right.clone())))
        },
        Interaction::Seq(left, right) => format!("seq({}, {})", to_maude_interaction(left), to_maude_interaction(right)),
        Interaction::Alt(left, right) => format!("alt({}, {})", to_maude_interaction(left), to_maude_interaction(right)),
        Interaction::Par(left, right) => format!("par({}, {})", to_maude_interaction(left), to_maude_interaction(right)),
        Interaction::LoopS(inner) => format!("loopS({})", to_maude_interaction(inner)),
        Interaction::Tensor(left, right) => format!("tensor({}, {})", to_maude_interaction(left), to_maude_interaction(right)),
    }
}


