use clap::ArgMatches;
use crate::anti_unfication::modulo_empty::generalisation_empty_theory::generalisation_empty_theory;
use crate::global_counter::counter::reset_counter;
use crate::terms::parsing::interface::parse_file;

pub fn cli_lgg(matches: &ArgMatches) {
    reset_counter();

    let file = matches.value_of("file").unwrap();

    match parse_file(file) {
        Ok((sig,t1,t2)) => {
            let lgg = generalisation_empty_theory(&t1,&t2);

            println!("Generalisation successful");

            println!("{}", lgg);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}

