use std::fs;
use std::path::{Path, PathBuf};
use std::time::{ Instant};
use crate::benchmark_fm_26::benchmark_ouput::{BenchmarkOutput, Line};
use crate::benchmark_fm_26::error::BenchmarkError;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;

#[derive(Clone,Debug)]
pub struct Locals{
    pub normalized: Vec<Interaction>,
    pub mutated: Vec<Interaction>,
}
#[derive(Clone,Debug)]
pub struct Benchmark{
    pub global_interactions: Vec<(String,GeneralContext,Interaction)>,
    pub nb_local_rewrites: usize,
    pub nb_lifelines_partitions: usize,
    pub timout_secs: Option<f64>,
    pub output: BenchmarkOutput,
    pub axioms: Vec<Axioms>
}

impl Benchmark {
    #[allow(dead_code)]
    pub fn new( global_interactions: Vec<(String,GeneralContext,Interaction)>,nb_local_rewrites:usize, nb_lifelines_partitions: usize,timout_secs:Option<f64>, output: BenchmarkOutput, axioms: Vec<Axioms>) -> Self{
        Benchmark{ global_interactions,nb_local_rewrites, nb_lifelines_partitions, timout_secs,output, axioms }
    }


    pub fn init(global_models_folder: &str,nb_local_rewrites:usize, nb_lifelines_partitions: usize,timout_secs: Option<f64>, axioms: Vec<Axioms>)->Result<Benchmark,BenchmarkError>{
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
                global_interactions.sort_by(|a,b| a.0.cmp(&b.0));

                Ok(Benchmark{ global_interactions, nb_local_rewrites, nb_lifelines_partitions,timout_secs,output: BenchmarkOutput::new(), axioms })
            },
            Err(e)=>{
                    Err(BenchmarkError::FolderAccessError(e.to_string()))
            }
        }

    }

    pub fn get_local_interactions(gen_ctx: &GeneralContext,local: &Vec<Interaction>,nb_local_rewrites:usize)->Locals{

        let mut local_normalized: Vec<Interaction> = Vec::new();
        let mut local_mutated: Vec<Interaction> = Vec::new();

        for local in local.iter(){
            local_normalized.push(local.iat_canonize(gen_ctx));
        }

        for (ct,local) in local.iter().enumerate(){
            local_mutated.push(local.random_rewrites(nb_local_rewrites+ct).unwrap());
        }

        Locals{normalized: local_normalized,mutated: local_mutated,}
    }

    pub fn run(&mut self,draw:bool,alpuente:bool, verbose:bool,millis:bool)->Result<(), BenchmarkError>{
        let output_dir = "Benchmark_Output";
        fs::remove_dir_all(output_dir).ok();
        fs::create_dir(output_dir).ok();

        if draw{
            let _ = self.create_dir_tree(draw);
        }



        for (name,gen_ctx,global_interaction) in self.global_interactions.iter() {

            let canon_global = global_interaction.iat_canonize(gen_ctx).clean_gates(); //Important to clean gates here
            //FOR EACH GLOBAL
            let mut result_vec_per_partition = Vec::new();
            let mut gates_vec = Vec::new();

            let int_dir = format!("{}/{}", output_dir, name);

            if draw {
                let input_global_dir = format!("{}/input_global_interaction", &int_dir);
                draw_model(gen_ctx, name, &input_global_dir, global_interaction);
            }
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //// RANDOM DECOMPOSITION
            let all_locals = global_interaction.random_decompose_into_two_ints(self.nb_lifelines_partitions);

            //Random decomposition

            for (ct_partition,locals_original) in all_locals.iter().enumerate() { // CYCLE

                let partition_dir = format!("{}/Partition {}", int_dir, ct_partition);
                let locals_dir = format!("{}/original locals", partition_dir);
                let normalized_int_dir = format!("{}/with normalized locals",&partition_dir);
                let norm_input_local_dir = format!("{}/normalized local interactions",&normalized_int_dir);
                let norm_result_gf = format!("{}/result with greedy fail",&normalized_int_dir);
                let norm_result_non_gf = format!("{}/result without greedy fail",&normalized_int_dir);
                let mutated_int_dir = format!("{}/with mutated locals",&partition_dir);
                let mutated_local_dir = format!("{}/mutated local interactions",&mutated_int_dir);
                let mut_result_gf = format!("{}/result with greedy fail",&mutated_int_dir);
                let mut_result_non_gf = format!("{}/result without greedy fail",&mutated_int_dir);
                if draw{
                    fs::create_dir_all(&locals_dir).ok();
                    //Drawing locals
                    for (ct, local) in locals_original.iter().enumerate() {
                        let local_name = format!("i{}", ct + 1);
                        draw_model(gen_ctx, &local_name, &locals_dir, local);
                    }

                    fs::create_dir_all(&normalized_int_dir).ok();
                    fs::create_dir_all(&norm_input_local_dir).ok();
                    fs::create_dir_all(&norm_result_gf).ok();
                    fs::create_dir_all(&norm_result_non_gf).ok();

                    fs::create_dir_all(&mutated_int_dir).ok();
                    fs::create_dir_all(&mutated_local_dir).ok();
                    fs::create_dir_all(&mut_result_gf).ok();
                    fs::create_dir_all(&mut_result_non_gf).ok();
                }
                //Normalization + Mutation
                let locals = Self::get_local_interactions(gen_ctx,locals_original,self.nb_local_rewrites);

                //Recording of the number of gates for the partition
                gates_vec.push(locals.normalized[0].free_gates().len());

                if draw { // If draw flag is provided
                    for (ct, local) in locals.normalized.iter().enumerate() {
                        let local_name = format!("i{}", ct + 1);
                        draw_model(gen_ctx, &local_name, &norm_input_local_dir, local);
                    }
                }

                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //// MUTATION


                if draw { // If draw flag is provided
                    for (ct, local) in locals.mutated.iter().enumerate() {
                        let local_name = format!("i{}", ct + 1);
                        draw_model(gen_ctx, &local_name, &mutated_local_dir, local);
                    }
                }

                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //// NORMALIZED LOCALS

                ////////////////////////////////////////////////////////////////////////////
                //// COMPOSITION WITH GREEDY-FAIL
                let time = Instant::now();
                let duration_norm_1 = match Interaction::compose(&locals.normalized[0], &locals.normalized[1], alpuente, verbose, true,self.timout_secs,&self.axioms) {
                    Ok(result_int) => {
                        let elapsed = time.elapsed().as_secs_f64();

                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }
                        //Drawing of the result
                        /////////////////////////////////////////////////////////////////

                        if draw {
                            draw_model(gen_ctx, "result", &norm_result_gf, &result_int.iat_canonize(gen_ctx));
                        }

                        //////////////////////////////////////////////////////////////////
                        Some(elapsed)
                    },
                    Err(CompositionError::TimedOut)=>{
                        None
                    }
                    Err(e) => {
                        return Err(BenchmarkError::CompositionError(e.to_string()));
                    }


                };

                ////////////////////////////////////////////////////////////////////////////
                //// COMPOSITION WITHOUT GREEDY-FAIL
                let time = Instant::now();
                let duration_norm_2 = match Interaction::compose(&locals.normalized[0], &locals.normalized[1], alpuente, verbose, false,self.timout_secs,&self.axioms) {
                    Ok(result_int) => {
                        let elapsed = time.elapsed().as_secs_f64();

                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

                        //Drawing of the result

                        /////////////////////////////////////////////////////////////////

                        if draw {
                            draw_model(gen_ctx, "result", &norm_result_non_gf, &result_int.iat_canonize(gen_ctx));
                        }


                        //////////////////////////////////////////////////////////////////
                        Some(elapsed)
                    },
                    Err(CompositionError::TimedOut)=>{
                        None
                    }
                    Err(e) => {
                        return Err(BenchmarkError::CompositionError(e.to_string()));
                    }


                };


                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //// MUTATED LOCALS
                ////////////////////////////////////////////////////////////////////////////
                //// COMPOSITION WITH GREEDY-FAIL
                let time = Instant::now();
                let duration_mut_1 = match Interaction::compose(&locals.mutated[0], &locals.mutated[1], alpuente, verbose, true,self.timout_secs,&self.axioms) {
                    Ok(result_int) => {
                        let elapsed = time.elapsed().as_secs_f64();

                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }
                        //Drawing of the result
                        /////////////////////////////////////////////////////////////////

                        if draw {
                            draw_model(gen_ctx, "result", &mut_result_gf, &result_int.iat_canonize(gen_ctx));
                        }

                        //////////////////////////////////////////////////////////////////
                        Some(elapsed)
                    },
                    Err(CompositionError::TimedOut)=>{
                        None
                    }
                    Err(e) => {
                        return Err(BenchmarkError::CompositionError(e.to_string()));
                    }


                };

                ////////////////////////////////////////////////////////////////////////////
                //// COMPOSITION WITHOUT GREEDY-FAIL
                let time = Instant::now();
                let duration_mut_2 = match Interaction::compose(&locals.mutated[0], &locals.mutated[1], alpuente, verbose, false,self.timout_secs,&self.axioms) {
                    Ok(result_int) => {
                        let elapsed = time.elapsed().as_secs_f64();

                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

                        //Drawing of the result

                        /////////////////////////////////////////////////////////////////

                        if draw {
                            draw_model(gen_ctx, "result", &mut_result_non_gf, &result_int.iat_canonize(gen_ctx));
                        }


                        //////////////////////////////////////////////////////////////////
                        Some(elapsed)
                    },
                    Err(CompositionError::TimedOut)=>{
                        None
                    }
                    Err(e) => {
                        return Err(BenchmarkError::CompositionError(e.to_string()));
                    }


                };
                result_vec_per_partition.push((duration_norm_1, duration_norm_2, duration_mut_1, duration_mut_2));

            }


            //Adding a new line in the table for outputs
            let min_gate = gates_vec.iter().min().unwrap().clone();
            let max_gate = gates_vec.iter().max().unwrap().clone();

            let (av_composition_duration_norm_1,
                av_composition_duration_norm_2,
                av_composition_duration_mut_1,
                av_composition_duration_mut_2) = Line::averaging_results(&result_vec_per_partition, millis);
            let line = Line::new(name,
                                 global_interaction.size(),
                                 (min_gate,max_gate),
                                 av_composition_duration_norm_1,
                                 av_composition_duration_norm_2,
                                 av_composition_duration_mut_1,
                                 av_composition_duration_mut_2);

            self.output.add_line(&line);




            println!("Global interaction {} processed",name);


        }

        //creating the csv file
        self.output.sort();
        self.output.to_csv(output_dir,millis);

        Ok(())
    }


    pub fn create_dir_tree(&self,draw:bool)->Result<(),std::io::Error>{
        let output_dir = "Benchmark_Output";
        //fs::remove_dir_all(output_dir).ok();
        //fs::create_dir(output_dir).ok();

        for (name,_,_) in self.global_interactions.iter() {
            let int_dir = format!("{}/{}",output_dir,name);
            fs::create_dir_all(&int_dir)?;

            if draw{
                let input_global_dir = format!("{}/input_global_interaction",&int_dir);
                fs::create_dir_all(&input_global_dir)?;
                /*
                for ct_partition in 0..self.nb_lifelines_partitions{
                    let partition_dir = format!("{}/ Partition {}",int_dir,ct_partition);

                    let normalized_int_dir = format!("{}/with normalized locals",&partition_dir);
                    let norm_input_local_dir = format!("{}/normalized local interactions",&normalized_int_dir);
                    let norm_result_gf = format!("{}/result with greedy fail",&normalized_int_dir);
                    let norm_result_non_gf = format!("{}/result without greedy fail",&normalized_int_dir);
                    fs::create_dir_all(&normalized_int_dir)?;
                    fs::create_dir_all(&norm_input_local_dir)?;
                    fs::create_dir_all(&norm_result_gf)?;
                    fs::create_dir_all(&norm_result_non_gf)?;

                    let mutated_int_dir = format!("{}/with mutated locals",&partition_dir);
                    let mutated_local_dir = format!("{}/mutated local interactions",&mutated_int_dir);
                    let mut_result_gf = format!("{}/result with greedy fail",&mutated_int_dir);
                    let mut_result_non_gf = format!("{}/result without greedy fail",&mutated_int_dir);
                    fs::create_dir_all(&mutated_int_dir)?;
                    fs::create_dir_all(&mutated_local_dir)?;
                    fs::create_dir_all(&mut_result_gf)?;
                    fs::create_dir_all(&mut_result_non_gf)?;

                }

                 */

            }
        }





        Ok(())
    }



}

