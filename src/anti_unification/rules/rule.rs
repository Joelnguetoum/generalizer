

#[derive(Clone, Debug)]
pub enum Rule {
    Decompose,
    DecomposeC,
    DecomposeA,
    DecomposeAC,
    DecomposeU,
    DecomposeCU,
    DecomposeAU,
    DecomposeACU,
    Solve,
    Recover,
    ConstrainedSolve,
    ConstrainedRecover,
    SolveFail,
    GreedySolveFail,
    ExpandULeft,
    ExpandURight,
    ExpandUBoth,
    ExpandUBothDecompose,
    ExpandUSameLeft,
    ExpandUSameRight,
    ExpandUSameBoth,
}


impl Rule {
    pub fn name(&self) -> &'static str {
        match self {
            Rule::Decompose => "Decompose",
            Rule::DecomposeC => "Decompose_C",
            Rule::DecomposeA => "Decompose_A",
            Rule::DecomposeAC => "Decompose_AC",
            Rule::DecomposeU => "Decompose_U",
            Rule::DecomposeCU => "Decompose_CU",
            Rule::DecomposeAU => "Decompose_AU",
            Rule::DecomposeACU => "Decompose_ACU",
            Rule::Solve => "Solve",
            Rule::Recover => "Recover",
            Rule::ExpandULeft => "Expand_U_Left",
            Rule::ExpandURight => "Expand_U_Right",
            Rule::ConstrainedSolve => "Constrained-Solve",
            Rule::ConstrainedRecover => "Constrained-Recover",
            Rule::SolveFail => "Solve-Fail",
            Rule::GreedySolveFail => "Solve-Fail",
            Rule::ExpandUBoth => "Expand_U_Both",
            Rule::ExpandUBothDecompose => "Expand_U_BothDecompose",
            Rule::ExpandUSameLeft => "Expand_U_Left",
            Rule::ExpandUSameRight => "Expand_U_Right",
            Rule::ExpandUSameBoth => "Expand_U_RightBoth",
        }
    }
}