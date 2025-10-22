use crate::generaliser::generaliser::Generaliser;

pub fn print_generalisers(gen_vec: &Vec<Generaliser>,verbose: bool, dot: bool) {

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
                let file_name = format!("output{}",ct);

                hist.create_computation_graph(file_name.as_str()).ok();
            }
        }

        println!("========================================");
    }
}