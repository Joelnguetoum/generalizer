use std::fs;
use crate::benchmark_fm_26::benchmark_fm_26::Benchmark;
use crate::benchmark_fm_26::benchmark_ouput::Line;
use crate::benchmark_fm_26::error::BenchmarkError;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

impl Benchmark{
    pub fn run_step_3(&mut self,_draw:bool,_alpuente:bool, _verbose:bool,millis:bool)->Result<(),BenchmarkError>{
        let output_dir = "Benchmark_Output";

        if let Err(_) =  fs::exists(output_dir){
            panic!("The output directory {} does not exist. Please execute the first step of the Benchmark before the step 2",output_dir);
        }

        for (name,gen_ctx,global_interaction) in self.global_interactions.iter() {
            let canon_global = global_interaction.iat_canonize(gen_ctx).clean_gates(); //Important to clean gates here
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
                //println!("Local interaction parsing successful");
                //Recording of the number of gates for the partition
                gates_vec.push(locals.normalized[0].free_gates().len());


                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////////////
                //// NORMALIZED LOCALS

                ////////////////////////////////////////////////////////////////////////////
                //// COMPOSITION WITH GREEDY-FAIL
                let duration_norm_1 = match Self::parse_result(gen_ctx,&norm_result_gf) {
                    Ok((result_int,elapsed)) => {
                        if result_int.iat_canonize(gen_ctx).clean_gates() != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
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
                let duration_norm_2 = match Self::parse_result(gen_ctx,&norm_result_non_gf) {
                    Ok((result_int,elapsed)) => {

                        if result_int.iat_canonize(gen_ctx).clean_gates() != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

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

                let duration_mut_1 = match Self::parse_result(gen_ctx,&mut_result_gf) {
                    Ok((result_int,elapsed)) => {

                        if result_int.iat_canonize(gen_ctx).clean_gates() != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

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
                let duration_mut_2 = match Self::parse_result(gen_ctx,&mut_result_non_gf) {
                    Ok((result_int,elapsed)) => {

                        if result_int.iat_canonize(gen_ctx).clean_gates() != canon_global {
                            return Err(BenchmarkError::CompositionResultMismatch);
                        }

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




            println!("Normal form Checking for the global interaction {} processed",name);


        }


        //creating the csv file
        self.output.sort();
        self.output.to_csv_step_3(output_dir,millis);

        Ok(())
    }

    pub fn parse_result(gen_ctx:&GeneralContext, result_dir:&str)-> Result<(Interaction,f64),CompositionError>{



        match (parse_hif_file(gen_ctx,&format!("{}/result.hif",result_dir)),
               fs::read_to_string(format!("{}/time.txt",result_dir))){
            (Ok(result_int),Ok(v)) => {

                return Ok((result_int,v.parse::<f64>().unwrap()));
            },

            _ => {
                return Err(CompositionError::TimedOut);
            }
        }
    }

}