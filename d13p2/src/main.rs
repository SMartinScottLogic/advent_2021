use d13p2::load;

fn main() {
    let mut solution = load("input.d13p1.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
