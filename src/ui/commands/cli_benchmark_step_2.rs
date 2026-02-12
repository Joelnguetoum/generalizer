use clap::ArgMatches;
use crate::benchmark_fm_26::benchmark_fm_26::Benchmark;
use crate::terms::function::Axioms;

pub fn cli_benchmark_step_2(matches: &ArgMatches) {
    
    let folder = matches.value_of("folder").unwrap();
    let nb_mutations = matches.value_of("nb_mutations").unwrap().parse::<usize>().unwrap();
    let nb_lifelines_partitions = matches.value_of("nb_lifelines_partitions").unwrap().parse::<usize>().unwrap();
    let timeout = matches.value_of("timout_secs").unwrap().parse::<f64>().unwrap();

    let alpuente = matches.is_present("alpuente");
    let verbose = matches.is_present("verbose");
    let draw = matches.is_present("draw");
    let millis = matches.is_present("milliseconds");


    //println!("Max number of partitions {}",nb_lifelines_partitions);
    //println!("Number of mutations {}",nb_mutations);



    match Benchmark::init(folder,nb_mutations,nb_lifelines_partitions,Some(timeout),Axioms::acu()){
        Ok(mut benchmark) => {

            match benchmark.run_step_2(draw,alpuente,verbose,/*greedyfail,*/millis){
                Ok(_) => {
                    println!("Step 2 of the benchmark finished successfully");
                },
                Err(e) => {
                    println!("Step 2 of the benchmark failure: {:?}",e);
                }
            }
        },
        Err(e) => {
            println!("Failure of execute the Benchmark: {:?}",e);
        }
    }
    
    
}