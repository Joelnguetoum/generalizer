use clap::ArgMatches;
use crate::benchmark_fm_26::benchmark_fm_26::Benchmark;

pub fn cli_benchmark(matches: &ArgMatches) {
    let folder = matches.value_of("folder").unwrap();
    let nb_mutations = matches.value_of("nb_mutations").unwrap().parse::<usize>().unwrap();
    let nb_comp_per_global = matches.value_of("nb_composition_per_global").unwrap().parse::<usize>().unwrap();

    let alpuente = matches.is_present("alpuente");
    let verbose = matches.is_present("verbose");
    let greedyfail = matches.is_present("greedyfail");
    let draw = matches.is_present("draw");
    let millis = matches.is_present("milliseconds");

    match Benchmark::init(folder,nb_mutations,nb_comp_per_global){
        Ok(mut benchmark) => {

             match benchmark.run(draw,alpuente,verbose,greedyfail,millis){
                 Ok(_) => {
                     println!("Benchmark finished successfully");
                 },
                 Err(e) => {
                     println!("Benchmark failure: {:?}",e);
                 }
             }
        },
        Err(e) => {
            println!("Failure of execute the Benchmark: {:?}",e);
        }
    }

}
