use std::process::{Command, Stdio};
use crate::interactions::syntax::interaction::Interaction;
use crate::maude::from_maude::parse_maude_interaction::parse_maude_interaction;
use crate::maude::maude_paths::INTERACTION_RANDOM_REWRITE_PYTHON_FILE;
use crate::maude::to_maude::to_maude_interaction::to_maude_interaction;


impl Interaction {

    pub fn random_rewrites(&self,nb_rewrites: usize) -> Option<Interaction> {
        let int_str = to_maude_interaction(self);

        let process = if let Ok(c) = Command::new("python3")
            .arg(INTERACTION_RANDOM_REWRITE_PYTHON_FILE)
            .arg(int_str.as_str())
            .arg(nb_rewrites.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn(){
            c
        }
        else {
            return None;
        };

        let output = process.wait_with_output().expect("Failed to read stdout");

        let result_str  = String::from_utf8_lossy(&output.stdout);

        let result_interaction = parse_maude_interaction(&result_str,false);

        Some(result_interaction)
    }

    /*
    pub fn normalise(&self,order:bool)->Option<Interaction>{
        let int_str = to_maude_interaction(self);

        let mut process = if let Ok(c) = Command::new("python3")
            .arg(INTERACTION_NORMALIZATION_PYTHON_FILE)
            .arg(int_str.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn(){
            c
        }
        else {
            return None;
        };

        let output = process.wait_with_output().expect("Failed to read stdout");

        let result_str  = String::from_utf8_lossy(&output.stdout);



        let result_interaction = parse_maude_interaction(&result_str,order);

        Some(result_interaction)
    }

     */




}


