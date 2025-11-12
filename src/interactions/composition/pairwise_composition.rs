use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;


impl Interaction {
    pub fn compose(i1: &Interaction, i2: &Interaction,alpuente:bool,verbose:bool) -> Result<Interaction, CompositionError> {
        let guideline = Guideline::get_guideline(i1, i2)?;

        let t1 = i1.to_term(&guideline);
        let t2 = i2.to_term(&guideline);

        let mut process = GeneralisationProcess::init_process(&t1, &t2);
        //let mut process = GeneralisationEngine::init_engine(&t1,&t2);

        match process.constrained_generalise(alpuente, verbose) {
            Ok(clggs) => {
                let gen = clggs[0].clone();

                let res = gen.merge(&guideline)?;

                Ok(res)
            }
            Err(_e) => {
                return Err(CompositionError::CompositionFailure);
            }
        }
    }
}
