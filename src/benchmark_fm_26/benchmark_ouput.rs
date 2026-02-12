use std::fs::File;



#[derive(Clone,Debug)]
pub struct Line{
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub gates_range: (usize,usize),
    pub av_composition_duration_norm_with_greedy_fail: Option<f64>,
    pub av_composition_duration_norm_without_greedy_fail: Option<f64>,
    pub av_composition_duration_mut_with_greedy_fail: Option<f64>,
    pub av_composition_duration_mut_without_greedy_fail: Option<f64>,
}


impl Line{
    pub fn new(global_interaction_name: &String,
               global_interaction_size: usize,
               gates_range: (usize,usize),
               av_composition_duration_norm_with_greedy_fail: Option<f64>,
               av_composition_duration_norm_without_greedy_fail: Option<f64>,
               av_composition_duration_mut_with_greedy_fail: Option<f64>,
               av_composition_duration_mut_without_greedy_fail: Option<f64>

    )->Self{
        Line{global_interaction_name: global_interaction_name.clone(),
            global_interaction_size,
            gates_range,
            av_composition_duration_norm_with_greedy_fail,
            av_composition_duration_norm_without_greedy_fail,
            av_composition_duration_mut_with_greedy_fail,
            av_composition_duration_mut_without_greedy_fail,
        }
    }



   pub  fn averaging_results(vec: &Vec<(Option<f64>, Option<f64>,Option<f64>, Option<f64>)>, millis:bool) ->(Option<f64>, Option<f64>,Option<f64>, Option<f64>){

       let mut av1 = Some(0.0);
       let mut av2 = Some(0.0);
       let mut av3 = Some(0.0);
       let mut av4 = Some(0.0);
       for (a, b,c,d) in vec{

           av1 = match (&av1, &a){
               (Some(x),Some(y)) =>{
                    Some(x + y)
               },
               _ => None
           };

           av2 = match (&av2, &b){
               (Some(x),Some(y)) =>{
                   Some(x + y)
               },
               _ => None
           };

           av3 = match (&av1, &c){
               (Some(x),Some(y)) =>{
                   Some(x + y)
               },
               _ => None
           };

           av4 = match (&av2, &d){
               (Some(x),Some(y)) =>{
                   Some(x + y)
               },
               _ => None
           };
       }

       av1 = Self::averaging_and_round_duration(&av1, millis, vec.len());
       av2 = Self::averaging_and_round_duration(&av2, millis, vec.len());
       av3 = Self::averaging_and_round_duration(&av3, millis, vec.len());
       av4 = Self::averaging_and_round_duration(&av4, millis, vec.len());

       (av1, av2,av3,av4)
    }

    fn averaging_and_round_duration(av: &Option<f64>, millis:bool, total_len:usize) ->Option<f64>{
        if let Some(v) = av {
            if millis{
                let v1 = (v/(total_len as f64)) *1000.0; //Average + Conversion to ms
                Some((v1*1000.0).round() / 1000.0)  //Rounding
            }
            else{
                let v1 = v/(total_len as f64); //Average
                Some((v1*1000.0).round() / (total_len as f64 *1000.0))
            }
        }
        else{
            None
        }
    }
}
#[derive(Clone,Debug)]
pub struct BenchmarkOutput{
    pub results_benchmark: Vec<Line>,
}

impl BenchmarkOutput{
    pub fn new() -> BenchmarkOutput{
        BenchmarkOutput{ results_benchmark: Vec::new()}
    }


    pub fn add_line(&mut self, line: &Line){
        self.results_benchmark.push(line.clone());
    }

    pub fn sort(&mut self) {
        self.results_benchmark
            .sort_by(|a, b| a.global_interaction_name.cmp(&b.global_interaction_name));
    }

    pub fn to_csv_for_paper(&self,parent_dir: &str,millis:bool){
        let file_name = format!("{}/results.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing
        let mut duration1_str =  String::from("(Normalized locals) Av.Composition duration with GF");
        let mut duration2_str =  String::from("(Normalized locals) Av.Composition duration without GF");
        let mut duration3_str =  String::from("(Mutated locals) Av.Composition duration with GF");
        let mut duration4_str =  String::from("(Mutated locals) Av.Composition duration without GF");
        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
            duration4_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
            duration4_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Gates range",
            &duration1_str,
            &duration2_str,
            &duration3_str,
            &duration4_str]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                Self::custom_interval_string(line.gates_range), //line.gates_range.to_string(),
                Self::custom_expect(&line.av_composition_duration_norm_with_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_norm_without_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_mut_with_greedy_fail),
                format!("{}\\\\",Self::custom_expect(&line.av_composition_duration_mut_without_greedy_fail))]);
        }

        let _ = wtr.flush();


    }

    pub fn to_csv(&self,parent_dir: &str,millis:bool){
        let file_name = format!("{}/results_one_pass.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing
        let mut duration1_str =  String::from("(Normalized locals) Av.Composition duration with GF");
        let mut duration2_str =  String::from("(Normalized locals) Av.Composition duration without GF");
        let mut duration3_str =  String::from("(Mutated locals) Av.Composition duration with GF");
        let mut duration4_str =  String::from("(Mutated locals) Av.Composition duration without GF");
        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
            duration4_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
            duration4_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Gates range",
            &duration1_str,
            &duration2_str,
            &duration3_str,
            &duration4_str]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                Self::custom_interval_string(line.gates_range), //line.gates_range.to_string(),
                Self::custom_expect(&line.av_composition_duration_norm_with_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_norm_without_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_mut_with_greedy_fail),
                format!("{}",Self::custom_expect(&line.av_composition_duration_mut_without_greedy_fail))]);
        }

        let _ = wtr.flush();


    }


    pub fn to_csv_step_2(&self,parent_dir: &str,millis:bool){
        let file_name = format!("{}/results_step_2.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing
        let mut duration1_str =  String::from("(Normalized locals) Av.Composition duration with GF");
        let mut duration2_str =  String::from("(Normalized locals) Av.Composition duration without GF");
        let mut duration3_str =  String::from("(Mutated locals) Av.Composition duration with GF");
        let mut duration4_str =  String::from("(Mutated locals) Av.Composition duration without GF");
        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
            duration4_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
            duration4_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Gates range",
            &duration1_str,
            &duration2_str,
            &duration3_str,
            &duration4_str]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                Self::custom_interval_string(line.gates_range), //line.gates_range.to_string(),
                Self::custom_expect_step_2(&line.av_composition_duration_norm_with_greedy_fail),
                Self::custom_expect_step_2(&line.av_composition_duration_norm_without_greedy_fail),
                Self::custom_expect_step_2(&line.av_composition_duration_mut_with_greedy_fail),
                format!("{}",Self::custom_expect_step_2(&line.av_composition_duration_mut_without_greedy_fail))]);
        }

        let _ = wtr.flush();


    }

    pub fn to_csv_step_3(&self,parent_dir: &str,millis:bool){
        let file_name = format!("{}/results_step_3.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing
        let mut duration1_str =  String::from("(Normalized locals) Av.Composition duration with GF");
        let mut duration2_str =  String::from("(Normalized locals) Av.Composition duration without GF");
        let mut duration3_str =  String::from("(Mutated locals) Av.Composition duration with GF");
        let mut duration4_str =  String::from("(Mutated locals) Av.Composition duration without GF");
        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
            duration4_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
            duration4_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Gates range",
            &duration1_str,
            &duration2_str,
            &duration3_str,
            &duration4_str]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                Self::custom_interval_string(line.gates_range), //line.gates_range.to_string(),
                Self::custom_expect(&line.av_composition_duration_norm_with_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_norm_without_greedy_fail),
                Self::custom_expect(&line.av_composition_duration_mut_with_greedy_fail),
                format!("{}",Self::custom_expect(&line.av_composition_duration_mut_without_greedy_fail))]);
        }

        let _ = wtr.flush();


    }
    fn custom_interval_string(interval: (usize,usize))->String{
        if interval.0 < interval.1{
            format!("[{}, {}]", interval.0, interval.1)
        }
        else if interval.1 < interval.0{
            format!("[{}, {}]", interval.0, interval.1)
        }
        else{
            format!("{}", interval.0)
        }
    }
    pub fn custom_expect(op: &Option<f64>)->String{
        match op{
            Some(x) => format!("{}(Ok)",x),
            None => "timeout".to_string(),
        }
    }

    pub fn custom_expect_step_2(op: &Option<f64>)->String{
        match op{
            Some(x) => format!("{}",x),
            None => "timeout".to_string(),
        }
    }


}
