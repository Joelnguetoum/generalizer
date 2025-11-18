use std::fs;
use std::time::Instant;
use clap::ArgMatches;
use colored::Colorize;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;

pub fn cli_compose(matches: &ArgMatches) {
    let hsf_file = matches.value_of("hsf").unwrap();
    let int1_file = matches.value_of("hif1").unwrap();
    let int2_file = matches.value_of("hif2").unwrap();

    let gen_ctx = if let Ok(sig) = parse_hsf_file(hsf_file){
        sig
    }
    else{
        panic!("Could not parse HSF file");
    };

    let i = if let Ok(int) = parse_hif_file(&gen_ctx,int1_file){
        int
    }
    else{
        panic!("Could not parse HIF file");
    };

    let j = match parse_hif_file(&gen_ctx,int2_file) {
        Ok(int)=>{
            int.clone()
        },
        Err(e)=>{
            println!("{}",e);
            panic!("Could not parse HIF file");
        }
    };


    let verbose = matches.is_present("verbose");
    let alpuente = matches.is_present("alpuente");
    let greedy_fail = matches.is_present("greedyfail");

    let comp_dir = "Composition Output";
    let _ = fs::remove_dir_all(comp_dir).ok();
    let _ = fs::create_dir_all(comp_dir).ok();

    //Step 0: Drawing inputs
    let inputs_dir = format!("{}/inputs",comp_dir);
    let _ = fs::create_dir_all(inputs_dir.clone()).ok();
    draw_model(&gen_ctx,"i",&inputs_dir,&i);
    draw_model(&gen_ctx,"j",&inputs_dir,&j);

    //Step 1: Composition
    let time = Instant::now();

    match Interaction::compose(&i,&j,alpuente,verbose,greedy_fail) {
        Ok(comp_int) =>{
            let elapsed = time.elapsed().as_secs_f64();

            println!("{}", "Composition successful".to_string().green());
            println!("Duration: {} s", elapsed);

            let result_dir = format!("{}/result",comp_dir);
            let _ = fs::create_dir_all(result_dir.clone()).ok();
            draw_model(&gen_ctx,"result",&result_dir,&comp_int);


        },
        Err(e)=>{
            println!("Composition error: {:?}",e);
        }
    }


}
