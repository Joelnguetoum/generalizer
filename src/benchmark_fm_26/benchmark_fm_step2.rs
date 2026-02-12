use std::fs;
use std::time::Instant;
use crate::benchmark_fm_26::benchmark_fm_26::{Benchmark, Locals};
use crate::benchmark_fm_26::benchmark_ouput::{BenchmarkOutput, Line};
use crate::benchmark_fm_26::error::BenchmarkError;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

impl Benchmark{
    pub fn run_step_2(&mut self,draw:bool,alpuente:bool, verbose:bool,millis:bool)->Result<(),BenchmarkError>{
        let output_dir = "Benchmark_Output";

         if let Err(_) =  fs::exists(output_dir){
             panic!("The output directory {} does not exist. Please execute the first step of the Benchmark before the step 2",output_dir);
         }

        for (name,gen_ctx,global_interaction) in self.global_interactions.iter() {
            let _canon_global = global_interaction.iat_canonize(gen_ctx).clean_gates(); //Important to clean gates here
            //FOR EACH GLOBAL
            let mut result_vec_per_partition = Vec::new();
            let mut gates_vec = Vec::new();

            let int_dir = format!("{}/{}", output_dir, name);

            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //// RANDOM DECOMPOSITION
            let nb_partitions = global_interaction.nb_partitions_shuffled(self.nb_lifelines_partitions);

            //Random decomposition

            for ct_partition in 0..nb_partitions { // CYCLE

                let partition_dir = format!("{}/Partition {}", int_dir, ct_partition);
                let _locals_dir = format!("{}/original_locals", partition_dir);
                let normalized_int_dir = format!("{}/with_normalized_locals",&partition_dir);
                let norm_input_local_dir = format!("{}/normalized_local_interactions",&normalized_int_dir);
                let norm_result_gf = format!("{}/result_with_rule_fail",&normalized_int_dir);
                let norm_result_non_gf = format!("{}/result_without_rule_fail",&normalized_int_dir);
                let mutated_int_dir = format!("{}/with_mutated_locals",&partition_dir);
                let mutated_local_dir = format!("{}/mutated_local_interactions",&mutated_int_dir);
                let mut_result_gf = format!("{}/result_with_rule_fail",&mutated_int_dir);
                let mut_result_non_gf = format!("{}/result_without_rule_fail",&mutated_int_dir);

                //Normalization + Mutation
                let locals = Self::parse_locals(gen_ctx,&norm_input_local_dir,&mutated_local_dir);
                println!("For partition {} of {}: Local interaction parsing successful",ct_partition,name);
                //Recording of the number of gates for the partition
                gates_vec.push(locals.normalized[0].free_gates().len());


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
                        fs::write(&format!("{}/time.txt",norm_result_gf),elapsed.to_string()).unwrap();
                        /*
                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

                         */
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
                        fs::write(&format!("{}/time.txt",norm_result_non_gf),elapsed.to_string()).unwrap();
                        /*
                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

                         */

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
                        fs::write(&format!("{}/time.txt",mut_result_gf),elapsed.to_string()).unwrap();
                        /*
                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

                         */
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
                        fs::write(&format!("{}/time.txt",mut_result_non_gf),elapsed.to_string()).unwrap();
                        /*
                        if result_int.iat_canonize(gen_ctx) != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }
                         */

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

            //Here!! Partial csv
            BenchmarkOutput::csv_partition(name,&int_dir,millis,&result_vec_per_partition);

            println!("Composition of local interactions of {} completed for each partition of lifelines",name);
            println!("The duration of composition for each partitions are recorded in {}/{}_composition_durations.csv",int_dir,name);
            println!("");
            println!("-----------------------");
            println!("");
        }

        //creating the csv file
        self.output.sort();
        self.output.to_csv_step_2(output_dir,millis);

        Ok(())
    }


    pub fn parse_locals(gen_ctx:&GeneralContext, norm_int_folder: &str,mut_int_folder: &str)-> Locals{
        //println!("{}",norm_int_folder);
        let n_i1 = parse_hif_file(&gen_ctx,&format!("{}/i1.hif",norm_int_folder)).unwrap();
        let n_i2 = parse_hif_file(&gen_ctx,&format!("{}/i2.hif",norm_int_folder)).unwrap();

        let m_i1 = parse_hif_file(&gen_ctx,&format!("{}/i1.hif",mut_int_folder)).unwrap();
        let m_i2 = parse_hif_file(&gen_ctx,&format!("{}/i2.hif",mut_int_folder)).unwrap();

        Locals{ normalized:vec![n_i1,n_i2], mutated:vec![m_i1,m_i2]}
    }
}