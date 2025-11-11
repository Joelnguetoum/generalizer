use image::Rgb;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_LightGray, HCP_LightPurple, HCP_LightRed};

pub fn generate_color(seed: usize) -> Rgb<u8>{
    if seed == 1{
        Rgb(HCP_LightRed)
    }
    else if seed == 2{
        Rgb(HCP_LightPurple)
    }
    else if seed == 3{
        Rgb(HCP_LightGray)
    }
    else {
        let mut rng = StdRng::seed_from_u64((seed*11100011110011111) as u64);
        Rgb([rng.random(), rng.random(), rng.random()])
    }
}
