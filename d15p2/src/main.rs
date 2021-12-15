use log::info;

use d15p2::load;

fn main() {
    env_logger::init();

    //let mut solution = load("expected");
    let mut solution = load("input.d15p1.full");

    info!("solution: {:?}", solution);
    solution.analyse();
    info!("answer is {}", solution.answer());
}
