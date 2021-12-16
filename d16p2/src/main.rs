use d16p2::load;
use log::info;

fn main() {
    env_logger::init();

    let mut solution = load("input.d16p1.full");
    info!("solution: {:?}", solution);
    solution.analyse();
    info!("answer is {}", solution.answer());
}
