use image::Rgb;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_LIGHT_GRAY, HCP_LIGHT_PURPLE, HCP_LIGHT_RED};

pub fn generate_color(seed: usize) -> Rgb<u8>{
    if seed == 1{
        Rgb(HCP_LIGHT_RED)
    }
    else if seed == 2{
        Rgb(HCP_LIGHT_PURPLE)
    }
    else if seed == 3{
        Rgb(HCP_LIGHT_GRAY)
    }
    else {
        let mut rng = StdRng::seed_from_u64((seed*11100011110011111) as u64);
        Rgb([rng.random(), rng.random(), rng.random()])
    }
}
