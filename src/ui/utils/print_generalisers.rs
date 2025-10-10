use crate::generaliser::generaliser::Generaliser;

pub fn print_generalisers(gen_vec: &Vec<Generaliser>){

    println!("----------------------------------------");
    for (ct,gen) in gen_vec.iter().enumerate() {
        println!("Generaliser {}",ct);
        println!("{}",gen);
        println!("----------------------------------------");
    }
}