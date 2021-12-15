use template::load;

fn main() {
    env_logger::init();

    let mut solution = load("input.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
