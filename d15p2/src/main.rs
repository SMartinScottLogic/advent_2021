use d15p2::load;

fn main() {
    //let mut solution = load("expected");
    let mut solution = load("input.d15p1.full");

    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
