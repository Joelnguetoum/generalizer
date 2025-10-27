use std::fs;
use crate::anti_unification::generaliser::generaliser::Generaliser;

pub fn print_generalisers(gen_vec: &Vec<Generaliser>,verbose: bool, dot: bool) {

    let output_dir = "generaliser_output";
    fs::remove_dir_all(output_dir).ok();

    fs::create_dir(output_dir).ok();

    println!("========================================");
    for (ct,gen) in gen_vec.iter().enumerate() {
        println!("Generaliser {}",ct);
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