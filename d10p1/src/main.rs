use d10p1::load;

fn main() {
    let mut solution = load("input.d10p1.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
