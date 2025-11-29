use std::fs;
use crate::anti_unification::generalizer::generalizer::Generalizer;

pub fn print_generalizers(gen_vec: &Vec<Generalizer>, verbose: bool, dot: bool) {

    let output_dir = "generalizer_output";
    fs::remove_dir_all(output_dir).ok();

    fs::create_dir(output_dir).ok();

    println!("========================================");
    for (ct,gen) in gen_vec.iter().enumerate() {
        println!("Generalizer {}",ct);
        println!("{}",gen);

        if verbose {
            println!("\n Computation history: \n");
            if let Some(hist) = gen.clone().history{
                println!("{}",hist);
            }
        }


        if dot {

            if let Some(hist) = gen.clone().history{
                let file_name = format!("{}/output{}",output_dir,ct);

                hist.create_computation_graph(file_name.as_str()).ok();
            }
        }

        println!("========================================");
    }
}