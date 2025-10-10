use std::collections::HashMap;
use std::fmt;
use crate::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Substitution{
    pub map: HashMap<Variable, Term>,
}


impl Substitution {
    pub fn new()->Self{
        Self{ map: HashMap::new()}
    }

    pub fn from_map(map: &HashMap<Variable,Term>) -> Substitution{
        Substitution{map: map.clone()}
    }

    pub fn dom_vars(&self) -> Vec<Variable> {
        self.map.keys().cloned().collect()
    }

    pub fn insert(&mut self, var: &Variable, term: &Term) {
        self.map.insert(var.clone(), term.clone());
    }
}


impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut items: Vec<(&Variable, &Term)> = self.map.iter().collect();
        items.sort_by_key(|(v, _)| *v);
        let mut ct = items.len();

        result.push('{');
        for (variable, t) in items {
            result.push_str(&format!("{} --> {}", variable, t));

            ct -= 1;

            if ct >0{
                result.push(',');
            }
        }

        result.push('}');
        write!(f, "{}", result.trim_end())
    }
}

