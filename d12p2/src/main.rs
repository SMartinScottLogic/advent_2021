use d12p2::load;

fn main() {
    let mut solution = load("input.d12p1.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
