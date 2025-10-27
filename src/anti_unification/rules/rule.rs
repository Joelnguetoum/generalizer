use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::error::ConfigurationError;

#[derive(Clone, Debug)]
pub enum Rule {
    Decompose,
    DecomposeC,
    DecomposeA,
    DecomposeAC,
    DecomposeU,
    Solve,
    Recover,
    ConstrainedSolve,
    ConstrainedRecover,
    SolveFail,
    ExpandULeft,
    ExpandURight,
    ExpandUSame,
    ExpandDecomposeU,
}


impl Rule {
    pub fn name(&self) -> &'static str {
        match self {
            Rule::Decompose => "Decompose",
            Rule::DecomposeC => "Decompose_C",
            Rule::DecomposeA => "Decompose_A",
            Rule::DecomposeAC => "Decompose_AC",
            Rule::DecomposeU => "Decompose_U",
            Rule::ExpandDecomposeU => "Expand_Decompose_U",
            Rule::Solve => "Solve",
            Rule::Recover => "Recover",
            Rule::ExpandULeft => "Expand_U_Left",
            Rule::ExpandURight => "Expand_U_Right",
            Rule::ExpandUSame => "Expand_U_Same",
            Rule::ConstrainedSolve => "Constrained-Solve",
            Rule::ConstrainedRecover => "Constrained-Recover",
            Rule::SolveFail => "Solve-Fail"
        }
    }
}