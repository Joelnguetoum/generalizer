use std::fs;

pub fn print_file(file: &str){
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
                    🦀  Output \n\
                    ========================================\n"
            );
        }
        Err(e) => {
            println!("Error reading file {}", e);
        }
    }
}