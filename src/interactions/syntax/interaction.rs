use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::operators::Operator;

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub enum Interaction {
    Empty,
    Action(Action),
    Vp(Action,Action), //the order is meant to be emission-reception
    Seq(Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    LoopS(Box<Interaction>),
    Tensor(Box<Interaction>,Box<Interaction>),
}



impl Interaction {

    pub fn head_symbol(&self)->Option<Operator>{
        match self.clone() {
            Interaction::Empty => None,
            Interaction::Action(act)=>{None},
            Interaction::Vp(act1,act2)=>{None},
            Interaction::LoopS(_)=> {
                Some(Operator::LoopS)
            },
            Interaction::Seq(_,_)=>{
                Some(Operator::Seq)
            },
            Interaction::Par(_,_)=>{
                Some(Operator::Par)
            },
            Interaction::Alt(_,_)=>{
                Some(Operator::Alt)
            },
            Interaction::Tensor(_,_)=>{
                Some(Operator::Tensor)
            }
        }
    }

    pub fn common_gates(i:&Interaction,j:&Interaction)->HashSet<usize>{
        let gates_i = i.gates();
        let gates_j = j.gates();
        let common_gates = gates_i.intersection(&gates_j);

        let mut ret: HashSet<usize> = HashSet::new();

        for g in common_gates{
            ret.insert(*g);
        }

        ret
    }



    pub fn get_lifelines(&self) -> HashSet<usize> {
        match self.clone(){
            Interaction::Empty =>{
                HashSet::new()
            },
            Interaction::Action(act) => {
                HashSet::from([act.lf_id])
            },
            Interaction::Vp(act1,act2) => {
                HashSet::from([act1.lf_id,act2.lf_id])
            },
            Interaction::LoopS(box1) =>{
                let i1 = *box1;
                i1.get_lifelines()
            }
            Interaction::Seq(box1,box2)
            | Interaction::Alt(box1,box2)
            | Interaction::Par(box1,box2)
            | Interaction::Tensor(box1,box2)=> {
                let i1 = *box1;
                let i2 = *box2;

                let mut ret = HashSet::new();

                for lf in i1.get_lifelines(){
                    ret.insert(lf);
                }

                for lf in i2.get_lifelines(){
                    ret.insert(lf);
                }
                ret
            }
        }
    }

    pub fn project(&self,lifelines: &HashSet<usize>) -> Interaction {
        match self.clone() {
            Interaction::Empty => Interaction::Empty,
            Interaction::Action(act)=>{
                if lifelines.contains(&act.lf_id){
                    Interaction::Action(act)
                }
                else{
                    Interaction::Empty
                }
            },
            Interaction::Vp(act1,act2) => {
                if lifelines.contains(&act1.lf_id) && lifelines.contains(&act2.lf_id){
                    Interaction::Vp(act1,act2)
                }
                else if lifelines.contains(&act1.lf_id) && !lifelines.contains(&act2.lf_id){
                    Interaction::Action(act1)
                }
                else if !lifelines.contains(&act1.lf_id) && lifelines.contains(&act2.lf_id){
                    Interaction::Action(act2)
                }
                else{
                    Interaction::Empty
                }
            },
            Interaction::LoopS(box1) =>{
                let i = *box1;
                Interaction::wrap_loop(&i.project(lifelines))
            }
            Interaction::Seq(box1,box2)
            | Interaction::Alt(box1,box2)
            | Interaction::Par(box1,box2)
            | Interaction::Tensor(box1,box2)=> {
                let i = *box1;
                let j = *box2;

                Interaction::wrap_binary(&i.project(lifelines),&j.project(lifelines), &self.head_symbol().unwrap())
            }
        }

    }


    pub fn free_gates(&self) ->HashSet<usize>{

        match self.clone() {
            Interaction::Action(a)=>{
                if let Some(gate_id) = a.gate_id{
                    HashSet::from([gate_id])
                }
                else{
                    HashSet::new()
                }

            },
            Interaction::LoopS(box1)=>{
                let i = *box1;
                i.free_gates()
            },
            Interaction::Seq(box1, box2)
            |Interaction::Par(box1, box2)
            |Interaction::Alt(box1, box2)
            |Interaction::Tensor(box1, box2)=>{
                let i = *box1;
                let j = *box2;
                let mut set1 = i.free_gates();

                set1.extend(j.free_gates());

                set1
            },
            _ => HashSet::new()
        }
    }
    pub fn gates(&self)->HashSet<usize>{

        match self.clone() {
            Interaction::Empty =>{
                HashSet::new()
            },
            Interaction::Action(a)=>{
                if let Some(gate_id) = a.gate_id{
                    HashSet::from([gate_id])
                }
                else{
                    HashSet::new()
                }

            },
            Interaction::Vp(a,b)=>{
              match (a.gate_id,b.gate_id){
                  (Some(g1),Some(g2))=>{
                      HashSet::from([g1,g2])
                  },
                  (Some(g1),None)=>{
                      HashSet::from([g1])
                  },
                  (None,Some(g2))=>{
                      HashSet::from([g2])
                  },
                  (None,None)=>{
                      HashSet::new()
                  }
              }
            },
            Interaction::LoopS(box1)=>{
                let i = *box1;
                i.gates()
            },
            Interaction::Seq(box1, box2)
            |Interaction::Par(box1, box2)
            |Interaction::Alt(box1, box2)
            |Interaction::Tensor(box1, box2)=>{
                let i = *box1;
                let j = *box2;
                let mut set1 = i.gates();

                set1.extend(j.gates());

                set1
            }
        }
    }

    pub fn gates_vec(&self)->Vec<usize>{
        let set = self.gates();
        let mut v: Vec<_> = set.iter().copied().collect();
        v.sort();
        v
    }
    fn vec_cmp(v1: &Vec<usize>, v2: &Vec<usize>) -> Ordering {
        if v1.len()==0 && v2.len()==0 {
            return Ordering::Equal;
        }
        else if v1.len()==0 && v2.len()!=0 {
            return Ordering::Less;
        }
        else if v1.len()!=0 && v2.len()==0 {
            return Ordering::Greater;
        }
        else if v1[0] < v2[0] {
            return Ordering::Less;
        }
        else if v1[0] > v2[0] {
            return Ordering::Greater;
        }
        else{
            Self::vec_cmp(&Vec::from(&v1[1..]), &Vec::from(&v2[1..]))
        }
    }
    pub fn gates_cmp(&self,j:&Interaction)->Ordering{
        let gates_i = self.gates_vec();
        let gates_j = j.gates_vec();

        Self::vec_cmp(&gates_i, &gates_j)
    }


}


impl fmt::Display for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result = match self.clone() {
            Interaction::Empty =>{
                "Empty".to_string()
            },
            Interaction::Action(a)=>{
                format!("{}", a)
            },
            Interaction::Vp(a,b)=>{
                format!("Vp({},{})", a,b)
            },
            Interaction::LoopS(v)=>{
                format!("LoopS({})", *v)
            },
            Interaction::Seq(l,r)=>{
                format!("Seq({},{})", *l, *r)
            },
            Interaction::Par(l,r)=>{
                format!("Par({},{})", *l, *r)
            },
            Interaction::Alt(l,r)=>{
                format!("Alt({},{})", *l, *r)
            },
            Interaction::Tensor(l,r)=>{
                format!("Tensor({},{})", *l, *r)
            }
        };

        write!(f, "{}", result.trim_end())
    }
}
