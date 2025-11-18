pub mod terms;
mod global_counter;
mod ui;
mod matching;
mod anti_unification;
mod utils;
mod interactions;
mod benchmark_fm_26;
mod maude;

use crate::ui::generaliser_cli::generaliser_cli;

fn main() {

    generaliser_cli();

}
