use std::fs::File;



#[derive(Clone,Debug)]
pub struct Line{
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub gates_range: (usize,usize),
    pub av_composition_duration_with_greedy_fail: Option<f64>,
    pub av_composition_duration_without_greedy_fail: Option<f64>,
}


impl Line{
    pub fn new(global_interaction_name: &String,
               global_interaction_size: usize,
               gates_range: (usize,usize),
               av_composition_duration_with_greedy_fail: Option<f64>,
               av_composition_duration_without_greedy_fail: Option<f64>
              //verdict:bool
    )->Self{
        Line{global_interaction_name: global_interaction_name.clone(),
            global_interaction_size,
            gates_range,
            av_composition_duration_with_greedy_fail,
            av_composition_duration_without_greedy_fail}
    }



   pub  fn averaging_results(vec: &Vec<(Option<f64>, Option<f64>)>, millis:bool) ->(Option<f64>, Option<f64>){

       let mut av1 = Some(0.0);
       let mut av2 = Some(0.0);
       for (b,c) in vec{

           av1 = match (&av1, &b){
               (Some(x),Some(y)) =>{
                    Some(x + y)
               },
               _ => None
           };

           av2 = match (&av2, &c){
               (Some(x),Some(y)) =>{
                   Some(x + y)
               },
               _ => None
           };
       }


       av1 = if let Some(v) = av1 {
           if millis{
               let v1 = (v/(vec.len() as f64)) *1000.0; //Average + Conversion to ms
               Some((v1*1000.0).round() / 1000.0)  //Rounding
           }
           else{
               let v1 = v/(vec.len() as f64); //Average
               Some((v1*1000.0).round() / (vec.len() as f64 *1000.0))
           }
       }
       else{
           None
       };

       av2 = if let Some(v) = av2 {
           if millis{
               let v1 = (v/(vec.len() as f64)) *1000.0; //Average + Conversion to ms
               Some((v1*1000.0).round() / 1000.0)  //Rounding
           }
           else{
               let v1 = v/(vec.len() as f64); //Average
               Some((v1*1000.0).round() / (vec.len() as f64 *1000.0))
           }
       }
       else{
           None
       };

       (av1, av2)
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



    pub fn to_csv(&self,parent_dir: &str,millis:bool){
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
        let mut duration1_str =  String::from("Av.Composition duration with GF");
        let mut duration2_str =  String::from("Av.Composition duration without GF");
        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Gates range",
            &duration1_str,
            &duration2_str]);

        for line in &self.results_benchmark{
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                Self::custom_interval_string(line.gates_range), //line.gates_range.to_string(),
                Self::custom_expect(&line.av_composition_duration_with_greedy_fail),
                format!("{}\\\\",Self::custom_expect(&line.av_composition_duration_without_greedy_fail))]);
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
            Some(x) => format!("{}(\\greencheck)",x),
            None => "timeout".to_string(),
        }
    }


}
