use crate::terms::parsing::interface::parse_file;

pub mod terms;
pub mod substitution;
mod anti_unfication;
mod configuration;
mod generaliser;
mod global_counter;
mod constrained_anti_unification;
mod ui;
mod matching;

use std::env;
use crate::ui::generaliser_cli::generaliser_cli;

fn main() {

    generaliser_cli();

}
