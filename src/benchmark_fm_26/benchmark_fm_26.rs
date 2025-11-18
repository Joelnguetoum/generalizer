use std::fs;
use std::path::{Path, PathBuf};
use std::time::{ Instant};
use crate::benchmark_fm_26::benchmark_ouput::{BenchmarkOutput, Line};
use crate::benchmark_fm_26::error::BenchmarkError;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

#[derive(Clone,Debug)]
pub struct Benchmark{
    pub global_interactions: Vec<(String,GeneralContext,Interaction)>,
    pub nb_local_rewrites: usize,
    pub nb_comp_per_global_int: usize,
    pub output: BenchmarkOutput
}

impl Benchmark {
    #[allow(dead_code)]
    pub fn new( global_models: Vec<(String,GeneralContext,Interaction)>,nb_local_rewrites:usize, nb_comp_per_global_int: usize, output: BenchmarkOutput) -> Self{
        Benchmark{ global_interactions: global_models,nb_local_rewrites,nb_comp_per_global_int, output}
    }


    pub fn init(global_models_folder: &str,nb_local_rewrites:usize, nb_comp_per_global_int: usize,)->Result<Benchmark,BenchmarkError>{
         let folder_path = Path::new(global_models_folder);

        match fs::read_dir(&folder_path){
            Ok(sub_folders)=>{
                let mut global_interactions = Vec::new();


                for sub_folder in sub_folders{
                    let sub_folder_path = sub_folder.unwrap().path();
                    let sub_folder_name = sub_folder_path.file_name().unwrap().to_str().unwrap();

                    let mut gen_ctx = GeneralContext::new();
                    let mut interaction = Interaction::Empty;

                    let mut hsf: Option<PathBuf> = None;
                    let mut hif: Option<PathBuf> = None;

                    for entry in fs::read_dir(sub_folder_path.clone()).unwrap() {
                        let entry = entry.unwrap();
                        let path = entry.path();

                        match path.extension().and_then(|e| e.to_str()) {
                            Some("hsf") => hsf = Some(path),
                            Some("hif") => hif = Some(path),
                            _ => {}
                        }
                    }
                    if let Some(hsf_path) = hsf {
                        let hsf_path = hsf_path.as_path().to_str().unwrap();

                        if let Ok(ctx) = parse_hsf_file(hsf_path){
                            gen_ctx = ctx;
                        }
                        else{
                            return Err(BenchmarkError::HsfFileError(hsf_path.to_string()));
                        }
                    }

                    if let Some(hif_path) = hif {
                        let hif_path = hif_path.as_path().to_str().unwrap();

                        if let Ok(int) = parse_hif_file(&gen_ctx,hif_path){
                             interaction = int;
                        }
                        else{
                            return Err(BenchmarkError::HifFileError(hif_path.to_string()));
                        }
                    }

                    global_interactions.push((sub_folder_name.to_string(),gen_ctx, interaction));


                }

                Ok(Benchmark{ global_interactions,nb_local_rewrites, nb_comp_per_global_int,output: BenchmarkOutput::new()})
            },
            Err(e)=>{
                    Err(BenchmarkError::FolderAccessError(e.to_string()))
            }
        }

    }


    pub fn run(&mut self,draw:bool,alpuente:bool, verbose:bool, greedy_fail:bool,millis:bool)->Result<(), BenchmarkError>{
        let output_dir = "Benchmark Output";
        fs::remove_dir_all(output_dir).ok();
        fs::create_dir(output_dir).ok();

        if draw{
            let _ = self.create_dir_tree(draw);
        }



        for (name,gen_ctx,global_interaction) in self.global_interactions.iter() {



            //Step 1: Decomposition of global models, and local mutation
            let mut locals = global_interaction.random_decompose(2);

            //Drawing
            ///////////////////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////////////////
            let int_dir = format!("{}/{}",output_dir,name);
            if draw{
                let input_global_dir = format!("{}/input_global_interaction",&int_dir);
                let input_local_dir = format!("{}/local_interactions",&int_dir);

                draw_model(gen_ctx,name,&input_global_dir,global_interaction);
                for (ct,local) in locals.iter().enumerate(){
                    let local_name = format!("i{}", ct+1);

                    draw_model(gen_ctx,&local_name,&input_local_dir,local);

                }

            }
            ///////////////////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////////////////

            //Step 2: Mutation and composition
            let mut result_vec = Vec::new();
            for ct in 0..self.nb_comp_per_global_int{

                for ct in 0..locals.len(){

                    locals[ct] = locals[ct].random_rewrites(self.nb_local_rewrites).unwrap();

                }

                let time = Instant::now();
                match Interaction::compose(&locals[0],&locals[1],alpuente,verbose,greedy_fail) {
                    Ok(result_int) => {

                        let elapsed = time.elapsed().as_secs_f64();

                        result_vec.push((result_int.clone(),elapsed));

                        //Drawing of the result
                        /////////////////////////////////////////////////////////////////
                        //////////////////////////////////////////////////////////////////
                        /////////////////////////////////////////////////////////////////

                        if draw{
                            let result_dir = format!("{}/Result {}",int_dir,ct);
                            let _ = fs::create_dir_all(result_dir.clone()).ok();
                            draw_model(gen_ctx,"result",&result_dir,&result_int.iat_canonize(gen_ctx));
                        }

                        //////////////////////////////////////////////////////////////////
                        //////////////////////////////////////////////////////////////////
                        //////////////////////////////////////////////////////////////////

                    },
                    Err(e)=>{
                        return Err(BenchmarkError::CompositionError(e.to_string()));
                    }
                }

            }

            //Adding a new line in the table for outputs
            let (av_composition_duration,verdict) = Line::av_duration(gen_ctx, global_interaction,&result_vec,millis);
            let line = Line::new(name,global_interaction.size(),locals[0].free_gates().len(),av_composition_duration,verdict);
            self.output.add_line(&line);


            println!("Global interaction {} processed",name);

        }

        //creating the csv file
        self.output.to_csv(output_dir,millis);

        Ok(())
    }


    pub fn create_dir_tree(&self,draw:bool)->Result<(),std::io::Error>{
        let output_dir = "Benchmark Output";
        //fs::remove_dir_all(output_dir).ok();
        //fs::create_dir(output_dir).ok();

        for (name,_,_) in self.global_interactions.iter() {
            let int_dir = format!("{}/{}",output_dir,name);
            fs::create_dir_all(&int_dir)?;

            if draw{
                let input_global_dir = format!("{}/input_global_interaction",&int_dir);
                let input_local_dir = format!("{}/local_interactions",&int_dir);

                fs::create_dir_all(&input_global_dir)?;
                fs::create_dir_all(&input_local_dir)?;

            }
        }





        Ok(())
    }



}

