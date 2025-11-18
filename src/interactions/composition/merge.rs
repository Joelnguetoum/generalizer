use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::error::CompositionError::MergeFailure;
use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::action::ActionType;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::operators::Operator;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;


impl Generaliser {
    pub fn merge(&self,guideline: &Guideline)->Result<Interaction,CompositionError>{
        self.t.merge_rec(&self.sub1,&self.sub2,guideline)
    }
}
impl Term {
    pub fn merge_rec(&self,sub1: &Substitution,sub2:&Substitution,guideline: &Guideline)->Result<Interaction,CompositionError>{

        match self{
            Term::Variable(x)=>{
                if let (Some(t1), Some(t2)) = (sub1.get(x), sub2.get(x)) {
                    let i1 = t1.to_interaction();
                    let i2 = t2.to_interaction();

                    Ok(Interaction::wrap_binary(&i1,&i2,&Operator::Seq))
                }
                else{
                    Err(MergeFailure)
                }
            },
            Term::Function(f)=>{
                if self.is_special_constant(){
                    let g: usize = self.head_symbol_signature().name.parse().unwrap();



                    if let Some((a1,a2)) = guideline.map.get(&g){

                        //Here, make sure to have Emission-Reception in vp, in this order
                       let res =  match (&a1.action_type,&a2.action_type){
                            (ActionType::Emission,ActionType::Reception)=>{
                                Interaction::Vp(a1.clone(),a2.clone())
                            },
                            (ActionType::Reception,ActionType::Emission)=>{
                                Interaction::Vp(a2.clone(),a1.clone())
                            },
                             _ => panic!("{}", MergeFailure)
                        };
                        Ok(res)
                    }
                    else{
                         Err(MergeFailure)
                    }
                }
                else{
                    match (f.signature.arity,f.args.len()){
                        (0,0) => {
                            Ok(Interaction::parse_interaction(&f.signature.name))
                        },
                        (1,1) => {
                            match f.signature.name.as_str(){
                                "loopS"=>{
                                    Ok(Interaction::wrap_loop(&f.args[0].merge_rec(sub1,sub2,guideline)?))
                                },
                                _ => {
                                    panic!("Invalid function name during the conversion of a term into an interaction")
                                }
                            }
                        },
                        (2,2) => {
                            match f.signature.name.as_str(){
                                "seq"=>{
                                    let i1 = f.args[0].merge_rec(sub1,sub2,guideline)?;
                                    let i2 = f.args[1].merge_rec(sub2,sub1,guideline)?;

                                    Ok(Interaction::wrap_binary(&i1,&i2,&Operator::Seq))

                                },
                                "par"=>{
                                    let i1 = f.args[0].merge_rec(sub1,sub2,guideline)?;
                                    let i2 = f.args[1].merge_rec(sub2,sub1,guideline)?;

                                    Ok(Interaction::wrap_binary(&i1,&i2,&Operator::Par))
                                },
                                "alt"=>{
                                    let i1 = f.args[0].merge_rec(sub1,sub2,guideline)?;
                                    let i2 = f.args[1].merge_rec(sub2,sub1,guideline)?;

                                    Ok(Interaction::wrap_binary(&i1,&i2,&Operator::Alt))
                                },
                                "tensor"=>{
                                    let i1 = f.args[0].merge_rec(sub1,sub2,guideline)?;
                                    let i2 = f.args[1].merge_rec(sub2,sub1,guideline)?;

                                    Ok(Interaction::wrap_binary(&i1,&i2,&Operator::Tensor))
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
}

