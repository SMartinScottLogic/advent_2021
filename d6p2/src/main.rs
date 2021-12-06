use d6p2::load;

fn main() {
    let mut solution = load("input.small");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
