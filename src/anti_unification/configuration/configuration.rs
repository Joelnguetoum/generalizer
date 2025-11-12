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

        let t1_flat = t1.assoc_flatten();
        let t2_flat = t2.assoc_flatten();

        let init_active = Vec::from(&[AUT::new(x0.clone(),t1_flat.clone(), t2_flat.clone())]);

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

    pub fn apply_rule(&self, rule: Rule) -> Result<Vec<Configuration>, ConfigurationError> {
        match rule {
            Rule::Decompose => self.decompose(),
            Rule::DecomposeC => self.decompose_c(),
            Rule::DecomposeA => self.decompose_a(),
            Rule::DecomposeAC => self.decompose_ac(),
            Rule::ExpandULeftDecompose=> self.expand_u_left_decompose(),
            Rule::ExpandURightDecompose=> self.expand_u_right_decompose(),
            Rule::ExpandUSameLeftDecompose=> self.expand_u_same_left_decompose(),
            Rule::ExpandUSameRightDecompose=> self.expand_u_same_right_decompose(),
            Rule::Solve => Ok(vec![self.solve()?]),
            Rule::Recover => Ok(vec![self.recover()?]),
            Rule::ConstrainedSolve => Ok(vec![self.constrained_solve()?]),
            Rule::ConstrainedRecover => Ok(vec![self.constrained_recover()?]),
            //Rule::SolveFail=> Err(ConfigurationError::SolveFailed),
            Rule::SolveFail=> Ok(vec![]),
            _ =>{
                Err(ConfigurationError::InvalidRuleApplication)
            }
        }
    }



    pub fn get_applicable_rules_first_aut(&self,is_constrained_anti_unification: bool,alpuente: bool) -> Vec<Rule> {

        if self.active.is_empty() {
            return vec![];
        }

        let mut rules = Vec::new();

        if self.can_apply_decompose(){
            rules.push(Rule::Decompose);
        }
        if self.can_apply_decompose_c() {
            rules.push(Rule::DecomposeC);
        }
        if self.can_apply_decompose_a() {
            rules.push(Rule::DecomposeA);
        }
        if self.can_apply_decompose_ac(){
            rules.push(Rule::DecomposeAC);
        }
         if self.can_apply_expand_u_left_decompose()  {
            //SA
            rules.push(Rule::ExpandULeftDecompose);
        }
         if self.can_apply_expand_u_right_decompose() {
            //SA
            rules.push(Rule::ExpandURightDecompose);
        }
         if self.can_apply_expand_u_same_left_decompose() && !alpuente {
            //New rule
            rules.push(Rule::ExpandUSameLeftDecompose);
        }
        if self.can_apply_expand_u_same_right_decompose() && !alpuente {
            // New rule
            rules.push(Rule::ExpandUSameRightDecompose);
        }
        if self.can_apply_solve() && !is_constrained_anti_unification {
            rules.push(Rule::Solve);
        }
        if self.can_apply_recover() && !is_constrained_anti_unification {
            rules.push(Rule::Recover);
        }
        if self.can_apply_constrained_solve() && is_constrained_anti_unification {
            rules.push(Rule::ConstrainedSolve);
        }
        if self.can_apply_constrained_recover() && is_constrained_anti_unification {
            rules.push(Rule::ConstrainedRecover);
        }
        /////////////////////////
        /*
        if self.can_apply_solve_fail() && is_constrained_anti_unification {
            rules.push(Rule::SolveFail);
        }
         */
        ////////////////////////
        if self.can_apply_greedy_solve_fail() && is_constrained_anti_unification {
            rules.push(Rule::GreedySolveFail);
        }


        rules
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