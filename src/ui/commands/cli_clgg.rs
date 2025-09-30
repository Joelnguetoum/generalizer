use clap::ArgMatches;
use crate::anti_unfication::modulo_empty::generalisation_empty_theory::generalisation_empty_theory;
use crate::constrained_anti_unification::modulo_empty::constrained_generalisation_empty_theory::constrained_generalisation_empty_theory;
use crate::global_counter::counter::reset_counter;
use crate::terms::parsing::interface::parse_file;

pub fn cli_clgg(matches: &ArgMatches) {
    reset_counter();

    let file = matches.value_of("file").unwrap();


    match parse_file(file) {
        Ok((sig,t1,t2)) => {
            match constrained_generalisation_empty_theory(&t1,&t2){
                Some(clgg)=>{
                    println!("Constrained generalisation successful");

                    println!("{}", clgg);
                },
                None=>{
                    println!("Constrained generalisation Failed");
                    println!("No constrained generaliser could be found");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}
