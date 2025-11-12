

#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    Decompose,
    DecomposeC,
    DecomposeA,
    DecomposeAC,
    Solve,
    Recover,
    ConstrainedSolve,
    ConstrainedRecover,
    SolveFail,
    GreedySolveFail,
    ExpandULeftDecompose,
    ExpandURightDecompose,
    ExpandUSameLeftDecompose,
    ExpandUSameRightDecompose,
    //////////////////////
    /////////////////////
    /////////////////////
    //Implicit rules that are not directly applied
    ExpandULeft,
    ExpandURight,
    ExpandUSameLeft,
    ExpandUSameRight,
}




impl Rule {
    pub fn name(&self) -> &'static str {
        match self {
            Rule::Decompose => "Decompose",
            Rule::DecomposeC => "Decompose_C",
            Rule::DecomposeA => "Decompose_A",
            Rule::DecomposeAC => "Decompose_AC",
            Rule::Solve => "Solve",
            Rule::Recover => "Recover",
            Rule::ConstrainedSolve => "Constrained-Solve",
            Rule::ConstrainedRecover => "Constrained-Recover",
            Rule::SolveFail => "Solve-Fail",
            Rule::GreedySolveFail => "Greedy-Solve-Fail",
            Rule::ExpandULeftDecompose => "Expand_U_Left_Decompose",
            Rule::ExpandURightDecompose => "Expand_U_Right_Decompose",
            Rule::ExpandUSameLeftDecompose => "Expand_U_Same_Left_Decompose",
            Rule::ExpandUSameRightDecompose => "Expand_U_Same_Right_Decompose",
            ////////////////////////////////////
            /////Implicit rules
            Rule::ExpandULeft => "Expand_U_Left",
            Rule::ExpandURight => "Expand_U_Right",
            Rule::ExpandUSameLeft => "Expand_U_Same_Left",
            Rule::ExpandUSameRight => "Expand_U_Same_Right",
        }
    }
}