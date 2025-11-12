use std::collections::{HashMap, HashSet};
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::error::CompositionError::UniqueGatePropertyUnsatisfied;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::interaction::Interaction;

pub struct Guideline {
    pub(crate) map: HashMap<usize,(Action, Action)>,
}


impl Guideline {
    pub fn get_guideline(i1:&Interaction,i2:&Interaction)->Result<Guideline,CompositionError>{

        /*
           If either i1 or i2 do not have the unique gate property
         */
        if !i1.has_unique_gate_property() || !i2.has_unique_gate_property()
        {
            return Err(UniqueGatePropertyUnsatisfied);
        }

        //Shared gates
        let  shared_gates = Interaction::shared_gates(i1,i2);

        //Guideline
        //Perhaps could be done more efficiently???
        let mut map = HashMap::new();

        let actions1 = i1.get_actions();
        let actions2 = i2.get_actions();


            for action1 in &actions1{
                for action2 in &actions2{
                    if action1.gate_id.unwrap() == action2.gate_id.unwrap() && shared_gates.contains(&action1.gate_id.unwrap()) {
                        map.insert(action1.gate_id.unwrap(),(action1.clone(),action2.clone()));
                    }
                }

        }

        let guideline = Guideline{map };

        Ok(guideline)
    }
}


impl Interaction{


    pub fn shared_gates(i1:&Interaction,i2:&Interaction)->HashSet<usize>{
        let mut shared_gates = HashSet::new();

        let gates_i1 = i1.free_gates();
        let gates_i2 = i2.free_gates();
        for gt in gates_i1.intersection(&gates_i2).into_iter(){
            shared_gates.insert(*gt);
        }

        shared_gates
    }
    pub fn has_unique_gate_property(&self)->bool{
        let actions = self.get_actions();

        for action in &actions {
            for other_action in &actions {

                //First we avoid self-comparison
                if action == other_action {
                    continue;
                }

                //If there are two different action with the same gate, the unique gate property is violated
                if action.gate_id == other_action.gate_id {
                    return false
                }
            }
        }

        true
    }
    pub fn get_actions(&self) -> HashSet<Action> {
        match self.clone() {
            Interaction::Action(act)=>{HashSet::from([act.clone()])},

            Interaction::LoopS(box1)=> {
                let i1 = *box1.clone();
                i1.get_actions()
            },
            Interaction::Seq(box1,box2)
            |Interaction::Alt(box1,box2)
            |Interaction::Par(box1,box2)
            |Interaction::Tensor(box1,box2)
            =>{
                let i1 = box1.clone();
                let i2 = box2.clone();

                let mut ret = i1.get_actions();

                ret.extend(i2.get_actions());

                ret
            },
            _ => HashSet::new(),

        }
    }
}
