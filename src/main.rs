pub mod functions;
pub mod game;
pub mod packages;
pub mod player;

use crate::functions::run;

//  Begin `run` with specified game area size
fn main() {
    println!(
        "\nYou {}!",
        match run(5, 5) {
            true => "Win",
            false => "Lose",
        }
    );
}
