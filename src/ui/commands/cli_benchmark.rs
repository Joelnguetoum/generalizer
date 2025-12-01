use clap::ArgMatches;
use crate::benchmark_fm_26::benchmark_fm_26::Benchmark;
use crate::terms::function::Axioms;
use crate::ui::utils::print_axioms::print_axioms;

pub fn cli_benchmark(matches: &ArgMatches) {



    let folder = matches.value_of("folder").unwrap();
    let nb_mutations = matches.value_of("nb_mutations").unwrap().parse::<usize>().unwrap();
    let nb_lifelines_partitions = matches.value_of("nb_lifelines_partitions").unwrap().parse::<usize>().unwrap();
    let timeout = matches.value_of("timout_secs").unwrap().parse::<f64>().unwrap();

    let alpuente = matches.is_present("alpuente");
    let verbose = matches.is_present("verbose");
    let draw = matches.is_present("draw");
    let millis = matches.is_present("milliseconds");


    println!("Max number of partitions {}",nb_lifelines_partitions);
    println!("Number of mutations {}",nb_mutations);
    println!("Composition timout {}",timeout);

    let axioms = if matches.is_present("A") {
        vec![Axioms::A]
    }
    else if matches.is_present("C") {
        vec![Axioms::C]
    }
    else if matches.is_present("U") {
        vec![Axioms::U]
    }
    else if matches.is_present("AC") {
        vec![Axioms::A, Axioms::C]
    }
    else if matches.is_present("AU") {
        vec![Axioms::A, Axioms::U]
    }
    else if matches.is_present("CU") {
        vec![Axioms::C, Axioms::U]
    }
    else if matches.is_present("ACU") {
        vec![Axioms::A,Axioms::C, Axioms::U]
    }
    else if matches.is_present("S") {
        vec![]
    }
    else{ //By default ACU
        vec![Axioms::A, Axioms::C, Axioms::U]
    };


    //Printing of the axioms considered
    print_axioms(&axioms);

    match Benchmark::init(folder,nb_mutations,nb_lifelines_partitions,Some(timeout),axioms){
        Ok(mut benchmark) => {

             match benchmark.run(draw,alpuente,verbose,/*greedyfail,*/millis){
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
