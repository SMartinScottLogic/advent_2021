use d15p1::load;

fn main() {
    let mut solution = load("input.d15p1.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
