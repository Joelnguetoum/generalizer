pub mod terms;
mod global_counter;
mod ui;
mod matching;
mod anti_unification;
mod utils;


use crate::ui::generaliser_cli::generaliser_cli;

fn main() {

    generaliser_cli();

}
