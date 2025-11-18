use clap::{load_yaml, App};
use crate::ui::commands::cli_benchmark::cli_benchmark;
use crate::ui::commands::cli_clgg::cli_clgg;
use crate::ui::commands::cli_compose::cli_compose;
use crate::ui::commands::cli_lgg::cli_lgg;
use crate::ui::commands::cli_test::cli_test;
use crate::ui::utils::logo::print_logo;

pub fn generaliser_cli() -> i32{
    let yaml = load_yaml!("generaliser_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //let mut ret_print : Vec<String> = vec![];
    //let mut ret_code : u32 = 1;

    print_logo();

    if let Some(matches) = matches.subcommand_matches("lgg") {
        let _got = cli_lgg(matches);

    } else if let Some(matches) = matches.subcommand_matches("clgg") {
        let _got = cli_clgg(matches);


    }
    else if let Some(matches) = matches.subcommand_matches("compose") {
        let _got = cli_compose(matches);


    }
    else if let Some(matches) = matches.subcommand_matches("benchmark") {
        let _got = cli_benchmark(matches);

    }
    else if let Some(matches) = matches.subcommand_matches("test") {
        let _got = cli_test(matches);

    }
    /*else {
        //ret_print = vec!["".to_string(),"TYPE help or -h to get a summary of the utilities".to_string()];
        //ret_code = 0
    }
     */

    return 0;
}