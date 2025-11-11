use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;
impl Interaction{



    pub fn to_term(&self,guideline: &Guideline)->Term{
        match &self.clone(){
            Interaction::Empty =>{
                let sig = FunctionSignature::new(self.to_string(),0,vec![]);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Vp(a1,a2) =>{

                let label = format!("Vp({},{})",a1.action_label(),a2.action_label());

                let sig = FunctionSignature::new(label,0,vec![]);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Action(a)=>{

                let sig = if let Some(g) = a.gate_id{

                    if guideline.map.contains_key(&g){
                        FunctionSignature::new(g.to_string(),0,vec![Axioms::SpecialConst])
                    }
                    else{
                        FunctionSignature::new(a.action_label(),0,vec![])
                    }

                }
                else{
                     FunctionSignature::new(self.to_string(),0,vec![])
                };


                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::LoopS(box1)=>{
               let i1 = *box1.clone();

                let loop_sig: FunctionSignature = FunctionSignature::new("loopS".to_string(), 1, vec![]);


                let f = Function::new(&loop_sig, &vec![i1.to_term(&guideline)]);

                Term::Function(f)
            }
            Interaction::Seq(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let seq_sig: FunctionSignature = FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A, Axioms::U]);


                let f  = Function::new(&seq_sig, &vec![i1.to_term(&guideline), i2.to_term(&guideline)]);

                Term::Function(f)
            },
            Interaction::Par(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let par_sig: FunctionSignature = FunctionSignature::new("par".to_string(), 2, vec![Axioms::A, Axioms::U]);


                let f  = Function::new(&par_sig, &vec![i1.to_term(&guideline), i2.to_term(&guideline)]);

                Term::Function(f)
            },
            Interaction::Alt(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let alt_sig: FunctionSignature = FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A]);


                let f  = Function::new(&alt_sig, &vec![i1.to_term(&guideline), i2.to_term(&guideline)]);

                Term::Function(f)
            },
            Interaction::Tensor(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let tensor_sig: FunctionSignature = FunctionSignature::new("tensor".to_string(), 2, vec![Axioms::A, Axioms::U]);


                let f  = Function::new(&tensor_sig, &vec![i1.to_term(&guideline), i2.to_term(&guideline)]);

                Term::Function(f)
            }
        }
    }

}


