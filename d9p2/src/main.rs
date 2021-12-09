use d9p2::load;

fn main() {
    let mut solution = load("input.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
