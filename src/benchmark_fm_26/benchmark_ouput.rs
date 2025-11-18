use std::fs::File;

use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;


#[derive(Clone,Debug)]
pub struct Line{
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub nb_gates_locals: usize,
    pub av_composition_duration: f64,
    pub verdict:bool
}


impl Line{
    pub fn new(global_interaction_name: &String,
               global_interaction_size: usize,
               nb_gates_locals: usize,
               av_composition_duration: f64,
              verdict:bool)->Self{
        Line{global_interaction_name: global_interaction_name.clone(),global_interaction_size,nb_gates_locals,av_composition_duration,verdict}
    }



   pub  fn av_duration(gen_ctx: &GeneralContext,global: &Interaction,vec: &Vec<(Interaction,f64)>,millis:bool)->(f64,bool){
        let mut verdict = true;
        let mut av = 0.0;
       let global_canon = global.iat_canonize(gen_ctx);

        for (interaction,duration) in vec{
            av+= duration;

            if interaction.iat_canonize(gen_ctx) != global_canon{
                verdict = false;
            }
        }

        av = av/vec.len() as f64;

       if millis{
           av = av*1000.0;
       }

       let av_rounded = (av * 1000.0).round() / 1000.0;

        (av_rounded,verdict)
    }
}
#[derive(Clone,Debug)]
pub struct BenchmarkOutput{
    pub results_benchmark: Vec<Line>,
}

impl BenchmarkOutput{
    pub fn new() -> BenchmarkOutput{
        BenchmarkOutput{ results_benchmark: Vec::new()}
    }


    pub fn add_line(&mut self, line: &Line){
        self.results_benchmark.push(line.clone());
    }



    pub fn to_csv(&self,parent_dir: &str,millis:bool){
        let file_name = format!("{}/results.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing
        let mut duration_str =  String::from("Av.Composition duration");
        if millis {
            duration_str.push_str("(ms)");
        }
        else{
            duration_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Nb of gates locals",
            &duration_str,
            "Success verdict"]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                line.nb_gates_locals.to_string(),
                line.av_composition_duration.to_string(),
                line.verdict.to_string()]);
        }

        let _ = wtr.flush();


    }


}
