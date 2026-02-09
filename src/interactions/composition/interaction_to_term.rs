use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;




impl Interaction{



    pub fn to_term(&self,guideline: &Guideline,axioms: &Vec<Axioms>)->Term{
        match &self.clone(){
            Interaction::Empty =>{
                let sig = FunctionSignature::new(self.to_string(),0,vec![],None);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Vp(a1,a2) =>{

                let label = format!("Vp({},{})",a1.action_label(),a2.action_label());

                let sig = FunctionSignature::new(label,0,vec![],None);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Action(a)=>{

                let sig = if let Some(g) = a.gate_id{

                    if guideline.map.contains_key(&g){
                        FunctionSignature::new(g.to_string(),0,vec![Axioms::SpecialConst],None)
                    }
                    else{
                        FunctionSignature::new(a.action_label(),0,vec![],None)
                    }

                }
                else{
                     FunctionSignature::new(a.action_label(),0,vec![],None)
                };


                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::LoopS(box1)=>{
               let i1 = *box1.clone();

                let loop_sig: FunctionSignature = FunctionSignature::new("loopS".to_string(), 1, vec![],None);


                let f = Function::new(&loop_sig, &vec![i1.to_term(&guideline,axioms)]);

                Term::Function(f)
            }
            Interaction::Seq(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let seq_sig: FunctionSignature = if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::U){
                     FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A, Axioms::U],Some("Empty".to_string()))
                }
                else if axioms.contains(&Axioms::A) && !axioms.contains(&Axioms::U){
                     FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A],None)
                }
                else{
                     FunctionSignature::new("seq".to_string(), 2, vec![],None)
                };



                let f  = Function::new(&seq_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::Par(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();


                //let par_sig: FunctionSignature = FunctionSignature::new("par".to_string(), 2, vec![Axioms::A, Axioms::U]);
                let par_sig: FunctionSignature = if axioms.contains(&Axioms::U){
                    FunctionSignature::new("par".to_string(), 2, axioms.clone(),Some("Empty".to_string()))
                    
                }
                else{
                    FunctionSignature::new("par".to_string(), 2, axioms.clone(),None)
                };
                

                let f  = Function::new(&par_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::Alt(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let alt_sig: FunctionSignature = if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A,Axioms::C],None)
                }
                else if axioms.contains(&Axioms::A) && !axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A],None)
                }
                else if !axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::C],None)
                }
                else{
                    FunctionSignature::new("alt".to_string(), 2, vec![],None)
                };

                //let alt_sig: FunctionSignature = FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A]);


                let f  = Function::new(&alt_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(&guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::Tensor(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let tensor_sig: FunctionSignature = FunctionSignature::new("tensor".to_string(), 2, axioms.clone(),Some("Empty".to_string()));


                let f  = Function::new(&tensor_sig, &vec![i1.to_term(&guideline,axioms), i2.to_term(&guideline,axioms)]);

                Term::Function(f)
            }
        }
    }

}


