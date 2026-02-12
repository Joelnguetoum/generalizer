use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::anti_unification::error::ConfigurationError;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;

impl Interaction {
    pub fn compose(i1: &Interaction, i2: &Interaction,alpuente:bool,verbose:bool,greedy_fail:bool,timeout_secs: Option<f64>,axioms: &Vec<Axioms>) -> Result<Interaction, CompositionError> {
        let guideline = Guideline::get_guideline(i1, i2)?;

        let t1 = i1.to_term(&guideline,axioms);
        let t2 = i2.to_term(&guideline,axioms);

        let mut process = GeneralisationProcess::init_process(&t1, &t2);
        //let mut process = GeneralisationEngine::init_engine(&t1,&t2);

        match process.scp_generalize(true,alpuente, verbose, greedy_fail, timeout_secs) {
            Ok(clggs) => {
                let gen = clggs[0].clone();

                let res = gen.merge(&guideline)?;

                Ok(res.clean_gates())
            }
            Err(ConfigurationError::TimedOut)=>{
                Err(CompositionError::TimedOut)
            },
            Err(_e) => {
                Err(CompositionError::CompositionFailure)
            }
        }
    }
}
