use std::fmt::Display;
use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::history::History;
use crate::anti_unification::rules::rule::Rule;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub active : Vec<AUT>,
    pub store : Vec<AUT>,
    pub sub: Vec<Substitution>,
    pub x0 : Variable,
    pub history: History,
}


impl Configuration {
    pub fn new(active: Vec<AUT>, store: Vec<AUT>, sub: Vec<Substitution>,x0: Variable, history: History) -> Self {
        Self { active , store, sub,x0 , history }
    }

    pub fn init_conf(t1: &Term, t2: &Term) -> Configuration {
        let x0 = Variable::x_0();

        let sub = Vec::new();

        let init_active = Vec::from(&[AUT::new(x0.clone(),t1.clone(), t2.clone())]);

        Self::new(init_active,Vec::new(),sub,x0,History::new())
    }

    pub fn to_generaliser(&self) -> Generaliser{
        let mut sub1 = Substitution::new();
        let mut sub2 = Substitution::new();

        //println!("{:?}",self.store);


        for aut in self.active.iter() {
            sub1.insert(&aut.x.clone(),&aut.t1.clone());
            sub2.insert(&aut.x.clone(),&aut.t2.clone());
        }

        for aut in self.store.iter() {
            sub1.insert(&aut.x.clone(),&aut.t1.clone());
            sub2.insert(&aut.x.clone(),&aut.t2.clone());
        }

        let mut  t = Term::Variable(Variable::x_0());

        for sub in self.sub.iter() {
            t = t.apply_substitution(sub);
        }
        /*
        println!("Generaliser");
        println!("t: {}", t);
        println!("sub1: {}", sub1);
        println!("sub2: {}", sub2);

         */
        Generaliser::new_with_history(&t,&sub1,&sub2,&self.update_history(""))
    }

    pub fn update_history(&self,rule: &str)->History{
        let mut hist = self.history.clone();

        hist.add_config(&self,rule);
        hist
    }


    pub fn applicable_rule(&self,is_constrained_anti_unification: bool,alpuente: bool) -> Option<Rule> {
        if self.active.is_empty() {
            return None;
        }

        /*Greedy Optimisation: Prioritize Greedy Solve fail
        if self.can_apply_greedy_solve_fail() && is_constrained_anti_unification {
            return Some(Rule::GreedySolveFail);
        }
         */

        //Optimisation: Prioritize Solve fail
        /*
        if self.can_apply_solve_fail() && is_constrained_anti_unification {
            return Some(Rule::SolveFail);
        }

         */

        //Algorithm of Alpuente et al,
        if alpuente{
            return self.applicable_rule_alpuente(is_constrained_anti_unification);
        }


        //OUR ALGORITHM WITH THE NEW RULES
        if self.can_apply_decompose() && !self.can_apply_decompose_u(){
            Some(Rule::Decompose)
        } else if self.can_apply_decompose_c() && !self.can_apply_decompose_cu(){
            Some(Rule::DecomposeC)
        } else if self.can_apply_decompose_a() && !self.can_apply_decompose_au() {
            Some(Rule::DecomposeA)
        } else if self.can_apply_decompose_ac() && !self.can_apply_decompose_acu(){
            Some(Rule::DecomposeAC)
        } else if self.can_apply_decompose_u(){
            Some(Rule::DecomposeU)
        }
        else if self.can_apply_decompose_au(){
            Some(Rule::DecomposeAU)
        }
        else if self.can_apply_decompose_cu(){
            Some(Rule::DecomposeCU)
        }
        else if self.can_apply_decompose_acu(){
            Some(Rule::DecomposeACU)
        }else if self.can_apply_expand_u_both_decompose(){
            Some(Rule::ExpandUBothDecompose)
        }
        else if self.can_apply_solve() && !is_constrained_anti_unification {
            Some(Rule::Solve)
        } else if self.can_apply_recover() && !is_constrained_anti_unification {
            Some(Rule::Recover)
        } else if self.can_apply_constrained_solve() && is_constrained_anti_unification {
            Some(Rule::ConstrainedSolve)
        } else if self.can_apply_constrained_recover() && is_constrained_anti_unification {
            Some(Rule::ConstrainedRecover)
        }/*else if self.can_apply_solve_fail() && is_constrained_anti_unification {
            Some(Rule::SolveFail)
        }
        */
        else {
            None
        }

    }

    pub fn applicable_rule_alpuente(&self,is_constrained_anti_unification: bool) -> Option<Rule> {
        if self.active.is_empty() {
            return None;
        }

        if self.relaxed_can_apply_decompose(){
            Some(Rule::Decompose)
        } else if self.relaxed_can_apply_decompose_c() {
            Some(Rule::DecomposeC)
        } else if self.relaxed_can_apply_decompose_a() {
            Some(Rule::DecomposeA)
        } else if self.relaxed_can_apply_decompose_ac(){
            Some(Rule::DecomposeAC)
        }
        else if self.can_apply_expand_u_both(){
            Some(Rule::ExpandUBoth)
        }
        else if self.can_apply_solve() && !is_constrained_anti_unification {
            Some(Rule::Solve)
        } else if self.can_apply_recover() && !is_constrained_anti_unification {
            Some(Rule::Recover)
        } else if self.can_apply_constrained_solve() && is_constrained_anti_unification {
            Some(Rule::ConstrainedSolve)
        } else if self.can_apply_constrained_recover() && is_constrained_anti_unification {
            Some(Rule::ConstrainedRecover)
        }else if self.can_apply_solve_fail() && is_constrained_anti_unification {
            Some(Rule::SolveFail)
        }
        else {
            None
        }

    }
    pub fn apply_rule(&self, rule: Rule) -> Result<Vec<Configuration>, ConfigurationError> {
        match rule {
            Rule::Decompose => self.decompose(),
            Rule::DecomposeC => self.decompose_c(),
            Rule::DecomposeA => self.decompose_a(),
            Rule::DecomposeAC => self.decompose_ac(),
            Rule::DecomposeU => self.decompose_u(),
            Rule::DecomposeAU => self.decompose_au(),
            Rule::DecomposeCU => self.decompose_cu(),
            Rule::DecomposeACU => self.decompose_acu(),
            Rule::ExpandUBothDecompose => self.expand_u_both_decompose(),
            Rule::Solve => Ok(vec![self.solve()?]),
            Rule::Recover => Ok(vec![self.recover()?]),
            Rule::ExpandULeft=> self.expand_u_left(),
            Rule::ExpandURight=> self.expand_u_right(),
            Rule::ExpandUBoth=> self.expand_u_both(),
            Rule::ConstrainedSolve => Ok(vec![self.constrained_solve()?]),
            Rule::ConstrainedRecover => Ok(vec![self.constrained_recover()?]),
            Rule::SolveFail=> Err(ConfigurationError::SolveFailed),
            Rule::GreedySolveFail=> Err(ConfigurationError::SolveFailed),
            _ =>{
                Err(ConfigurationError::UnknownRule)
            }
        }
    }

    // Helper methods for configuration creation
    pub fn create_new_config(
        &self,
        active: Vec<AUT>,
        store: Vec<AUT>,
        sub: Vec<Substitution>,
        rule: &Rule,
    ) -> Configuration {
        Configuration::new(
            active,
            store,
            sub,
            self.x0.clone(),
            self.update_history(rule.name())
        )
    }

    pub fn with_fresh_aut_pair(
        &self,
        t1: Term,
        t1_prime: Term,
        t2: Term,
        t2_prime: Term,
    ) -> (Variable, Variable, AUT, AUT) {
        let x1 = Variable::fresh_variable();
        let x2 = Variable::fresh_variable();
        let aut1 = AUT::new(x1.clone(), t1, t1_prime);
        let aut2 = AUT::new(x2.clone(), t2, t2_prime);

        (x1, x2, aut1, aut2)
    }
}

impl Display for Configuration {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut active_string = String::new();
        let mut store_string = String::new();
        let mut sub_string = String::new();

        //Active set
        active_string.push('{');
        for aut in self.active.iter() {
            active_string.push_str(format!("{}, ",aut).as_str());
        }
        active_string.push('}');

        //Store set
        store_string.push('{');
        for aut in self.store.iter() {
            store_string.push_str(format!("{}, ",aut).as_str());
        }
        store_string.push('}');

        // Substitution

        if self.sub.is_empty(){
            sub_string.push_str("{}");
        }
        else{
            for sub in self.sub.iter() {
                sub_string.push_str(format!("{}",sub).as_str());
            }
        }




        //Total string

        let result = format!(
            "< {} |  {} | {} | {} >", active_string,store_string,sub_string, self.x0);

        write!(f, "{}", result)
    }
}