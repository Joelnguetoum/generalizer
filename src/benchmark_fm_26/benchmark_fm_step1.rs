use std::fs;
use crate::benchmark_fm_26::benchmark_fm_26::Benchmark;
use crate::benchmark_fm_26::error::BenchmarkError;
use crate::interactions::io::output::quick_drawing::draw_model;

impl Benchmark{
    pub fn run_step_1(&mut self,draw:bool,_alpuente:bool, _verbose:bool,_millis:bool)->Result<(),BenchmarkError>{
        let output_dir = "Benchmark_Output";
        fs::remove_dir_all(output_dir).ok();
        fs::create_dir(output_dir).ok();

        if draw{
            let _ = self.create_dir_tree(draw);
        }



        for (name,gen_ctx,global_interaction) in self.global_interactions.iter() {
            let _canon_global = global_interaction.iat_canonize(gen_ctx).clean_gates(); //Important to clean gates here
            //FOR EACH GLOBAL
            //let mut result_vec_per_partition = Vec::new();
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
            println!("");
            println!("----------------------------");
            println!("Chosen {} partitions of lifelines for Global interaction {}",all_locals.len(),name);
            //Random decomposition

            for (ct_partition,locals_original) in all_locals.iter().enumerate() { // CYCLE

                let partition_dir = format!("{}/Partition {}", int_dir, ct_partition);
                let locals_dir = format!("{}/original_locals", partition_dir);
                let normalized_int_dir = format!("{}/with_normalized_locals",&partition_dir);
                let norm_input_local_dir = format!("{}/normalized_local_interactions",&normalized_int_dir);
                let norm_result_gf = format!("{}/result_with_rule_fail",&normalized_int_dir);
                let norm_result_non_gf = format!("{}/result_without_rule_fail",&normalized_int_dir);
                let mutated_int_dir = format!("{}/with_mutated_locals",&partition_dir);
                let mutated_local_dir = format!("{}/mutated_local_interactions",&mutated_int_dir);
                let mut_result_gf = format!("{}/result_with_rule_fail",&mutated_int_dir);
                let mut_result_non_gf = format!("{}/result_without_rule_fail",&mutated_int_dir);
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

                println!("For partition {} of Global interaction {}: generated 1 local normalized interaction and 1 local mutated interaction",ct_partition,name);
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





            }

            println!("Projection and mutation/normalization of the Global interaction {} processed",name);


        }



        Ok(())
    }

}