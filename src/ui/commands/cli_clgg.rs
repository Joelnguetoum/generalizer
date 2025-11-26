use std::time::Instant;
use clap::ArgMatches;
use colored::Colorize;
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::global_counter::counter::reset_counter;
use crate::terms::parsing::interface::parse_file;
use crate::ui::utils::print_file::print_file;
use crate::ui::utils::print_generalisers::print_generalisers;

pub fn cli_clgg(matches: &ArgMatches) {
    reset_counter();

    let file = matches.value_of("file").unwrap();

    let verbose = matches.is_present("verbose");
    let dot = matches.is_present("dot");
    let alpuente = matches.is_present("alpuente");
    let greedy_fail = matches.is_present("greedyfail");

    /*
    let timout = if matches.is_present("timeout_secs"){
        Some(matches.value_of("timeout_secs").unwrap().parse::<f64>().unwrap())
    }else{
        None
    };
     */


    print_file(file,alpuente);

    match parse_file(file) {
        Ok((_sig,t1,t2)) => {
            let mut process = GeneralisationProcess::init_process(&t1,&t2);
            let time = Instant::now();
            //let mut process = GeneralisationEngine::init_engine(&t1,&t2);


            match process.constrained_generalise(alpuente,verbose,greedy_fail,None) {
                Ok(clggs)=>{

                    let elapsed = time.elapsed().as_secs_f64();

                    println!("{}", "Constrained generalisation successful".to_string().green());
                    println!("Duration: {} s", elapsed);

                    print_generalisers(&clggs,verbose,dot);

                },
                Err(e)=>{
                    let elapsed = time.elapsed().as_secs_f64();

                    println!("{}",e.to_string().red());
                    println!("Duration: {} s", elapsed);
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}
