use std::fmt::{format, Display};
use crate::configuration::aut::AUT;
use crate::configuration::history::History;
use crate::generaliser::generaliser::Generaliser;
use crate::substitution::substitution::Substitution;
use crate::substitution::variable::Variable;
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
        sub_string.push('{');
        for sub in self.sub.iter() {
            sub_string.push_str(format!("{}",sub).as_str());
        }
        sub_string.push('}');


        //Total string

        let result = format!(
            "< {} |  {} | {} | {} >", active_string,store_string,sub_string, self.x0);

        write!(f, "{}", result)
    }
}