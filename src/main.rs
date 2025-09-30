use crate::terms::parsing::interface::parse_file;

pub mod terms;
pub mod substitution;
mod anti_unfication;
mod configuration;
mod generaliser;
mod global_counter;
mod constrained_anti_unification;
mod ui;

use std::env;
use crate::anti_unfication::modulo_empty::generalisation_empty_theory::generalisation_empty_theory;
use crate::constrained_anti_unification::modulo_empty::constrained_generalisation_empty_theory::constrained_generalisation_empty_theory;
use crate::ui::generaliser_cli::generaliser_cli;

fn main() {

    generaliser_cli();

}
