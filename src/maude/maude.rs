

/*
pub struct Maude{
    pub process: Child
}


impl Maude{
    pub fn init(maude_path: &str, nowrap: bool) -> Maude{

        if nowrap{
            let mut process = if let Ok(c) = Command::new(maude_path)
                .arg("-no-banner")
                .arg("-trust")
                .arg("-no-wrap")
                .arg("-no-advise")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn(){
                c
            }
            else{
                println!("Opening Maude failure");
                panic!("Opening Maude failure");
                //return Err(format!("Failed to spawn maude run process: {}", maude_path));
            };

            Maude{process}
        }
        else{
            let mut process = if let Ok(c) = Command::new(maude_path)
                .arg("-no-banner")
                .arg("-trust")
                .arg("-no-advise")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn(){
                c
            }
            else{
                println!("Opening Maude failure");
                panic!("Opening Maude failure");
                //return Err(format!("Failed to spawn maude run process: {}", maude_path));
            };

            Maude{process}
        }

    }


    pub fn load_file(&mut self, maude_file: &str){
        let command = format!("load {} .", maude_file);
        if let Some(stdin) = self.process.stdin.as_mut() {
            writeln!(stdin, "{}", command).expect("Failed to write to Maude");
        }
    }
    pub fn close(&mut self){
        if let Some(stdin) = self.process.stdin.as_mut() {
            writeln!(stdin, "quit").expect("Failed to write quit command");
        }
        self.process.kill().expect("Failed to kill the maude process");
    }
    pub fn run_rewrite_command(&mut self, command: &str, filter_result: bool) -> Result<String,String>{
        if let Some(stdin) = self.process.stdin.as_mut() {
            writeln!(stdin, "{}", command).expect("Failed to write to Maude");
        }

        // Read output from Maude

        let mut output = String::new();
        if let Some(mut stdout) = self.process.stdout.as_mut() {

            let mut reader = BufReader::new(stdout);
            let mut buffer = String::new();
            for line in reader.lines() {
                let line = line.unwrap();
                //buffer.push_str(&line);
                // If not found, add this line to result and continue

                if line.contains("result"){
                    output.push_str(&line);
                    break;
                }

                if line.contains("No solution") {
                    break;
                }
            }
            //stdout.read_to_string(&mut output).expect("Failed to read Maude output");

        }

        //println!("Maude output: {:?}",output);
        //let filtered_output: String = clean_maude_output(&output);
        let filtered_output: String = Self::dummy_clean(&output);
        //println!("{}",filtered_output);

        if !filtered_output.is_empty() {
            return Ok(filtered_output);
        }

        if !filter_result {
            return Ok(output);
        }
        else{
            Err(format!("Maude session failed"))
        }
    }

    pub fn run_unify_command(&mut self, command: &str) -> Result<String,String>{
        if let Some(stdin) = self.process.stdin.as_mut() {
            writeln!(stdin, "{}", command).expect("Failed to write to Maude");
            // writeln!(stdin, "quit").expect("Failed to write quit command");
        }

        // Read output from Maude

        let mut output = String::new();
        if let Some(mut stdout) = self.process.stdout.as_mut() {

            let mut reader = BufReader::new(stdout);
            let mut buffer = String::new();
            for line in reader.lines() {
                let line = line.unwrap();
                //buffer.push_str(&line);
                // If not found, add this line to result and continue
                if line.contains("No more unifiers."){
                   break;
                }
                else if line.contains("No unifiers."){
                    return Err("Matching failure".to_string());
                }
                else{
                    output.push_str(&line);
                }
            }
            //stdout.read_to_string(&mut output).expect("Failed to read Maude output");

        }

        //println!("Maude output: {:?}",output);

        if !output.is_empty() {
            return Ok(output);
        }

        Err(format!("Maude session failed"))
    }
    pub fn dummy_clean(output: &String)->String{

        if let Some(s) = output.lines()
            .find(|line| line.contains("result"))
            .and_then(|result_line| {
                // Split at the last colon to get string2
                result_line.rsplit(':')
                    .next()
                    .map(|s| s.trim()) // Remove surrounding whitespace
            }){
            s.to_string()
        }
        else{
            "".to_string()
        }
    }
}

 */