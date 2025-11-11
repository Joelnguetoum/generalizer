use std::fs;

pub fn print_file(file: &str,alpuente: bool) {
    match fs::read_to_string(&file){
        Ok(s) => {
            println!(
                "\n========================================\n\
                    🦀  Input \n\
                    ========================================\n"
            );
            println!("{}", s);


            println!(
                "\n========================================\n\
                    🦀  Algorithm \n\
                    ========================================\n"
            );

            if alpuente{
                println!("Generalisation using the rules of Alpuente et al.");
                println!("CAVEAT: The rules of Alpuente et al. as implemented compute only linear generalisations modulo unit, and is incomplete");
            }
            else {
                println!("Generalisation using custom rules inspired from the work of Alpuente et al.");
            }
            println!(
                "\n========================================\n\
                    🦀  Output \n\
                    ========================================\n"
            );
        }
        Err(e) => {
            println!("Error reading file {}", e);
        }
    }
}