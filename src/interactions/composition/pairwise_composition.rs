use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::error::CompositionError::MergeFailure;
use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::operators::Operator;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;
use colored::Colorize;

impl Interaction {
    pub fn compose(i1: &Interaction, i2: &Interaction) -> Result<Interaction, CompositionError> {
        let guideline = Guideline::get_guideline(i1, i2)?;

        let t1 = i1.to_term(&guideline);
        let t2 = i2.to_term(&guideline);

        let mut process = GeneralisationProcess::init_process(&t1, &t2);

        match process.constrained_generalise(false, true) {
            Ok(clggs) => {
                let gen = clggs[0].clone();

                let res = gen.merge(&guideline)?;

                Ok(res)
            }
            Err(e) => {
                return Err(CompositionError::CompositionFailure);
            }
        }
    }
}
