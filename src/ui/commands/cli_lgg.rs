use std::time::Instant;
use clap::ArgMatches;
use crate::configuration::generalisation_process::GeneralisationProcess;
use crate::global_counter::counter::reset_counter;
use crate::terms::parsing::interface::parse_file;
use crate::ui::utils::logo::print_logo;
use crate::ui::utils::print_file::print_file;
use crate::ui::utils::print_generalisers::print_generalisers;

pub fn cli_lgg(matches: &ArgMatches) {
    reset_counter();

    let file = matches.value_of("file").unwrap();
    print_file(file);

    match parse_file(file) {
        Ok((sig,t1,t2)) => {
            let time = Instant::now();

            let mut process = GeneralisationProcess::init_process(&t1,&t2);

            let elapsed = time.elapsed().as_secs_f64();

            let lggs = process.generalise();


            println!("Generalisation successful");
            println!("Duration: {} s", elapsed);

            print_generalisers(&lggs);

        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}

