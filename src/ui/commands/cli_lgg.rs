use std::time::Instant;
use clap::ArgMatches;
use colored::Colorize;
use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;
use crate::global_counter::counter::reset_counter;
use crate::terms::parsing::interface::parse_file;
use crate::ui::utils::print_file::print_file;
use crate::ui::utils::print_generalisers::print_generalisers;

pub fn cli_lgg(matches: &ArgMatches) {
    reset_counter();

    let file = matches.value_of("file").unwrap();
    let verbose = matches.is_present("verbose");
    let dot = matches.is_present("dot");
    let alpuente = matches.is_present("alpuente");
    print_file(file,alpuente);

    match parse_file(file) {
        Ok((_sig,t1,t2)) => {
            let time = Instant::now();

            let mut process = GeneralisationProcess::init_process(&t1,&t2);
            let lggs = process.generalise(alpuente,verbose);

           // let mut process = GeneralisationEngine::init_engine(&t1,&t2);
           // process.generalise(alpuente,verbose);
           // let lggs = process.to_generalisers();

            let elapsed = time.elapsed().as_secs_f64();

            println!("{}", "Generalisation successful".to_string().green());
            println!("Duration: {} s", elapsed);

            print_generalisers(&lggs,verbose,dot);

        },
        Err(e) => {
            println!("Error: {}", e.to_string().red());
        }
    }


}

