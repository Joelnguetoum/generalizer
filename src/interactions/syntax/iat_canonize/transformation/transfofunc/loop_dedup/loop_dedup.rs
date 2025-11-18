use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::operators::Operator;

pub fn transfo_loops_deduplicate(interaction : &Interaction) -> Vec<Interaction> {
        match *interaction {
            Interaction::Seq(ref i1,ref i2) =>{

                match (*i1.clone(), *i2.clone()) {
                    (Interaction::LoopS(ref i11),Interaction::LoopS(ref i21)) if *i11 == *i21 => {
                        vec![*i1.clone()]
                    },
                    (Interaction::LoopS(ref i11),Interaction::Seq(ref i21,ref i22)) => {

                        match *i21.clone() {
                            Interaction::LoopS(ref i211) if *i11==*i211 => {
                                vec![Interaction::wrap_binary(&*i1.clone(),&*i22.clone(),&Operator::Seq)]
                            },
                            _ =>{
                                return vec![];
                            }
                        }

                    }
                    _ =>{
                        return vec![];
                    }
                }
            },
            _ => {
                return vec![];
            }
        }
}