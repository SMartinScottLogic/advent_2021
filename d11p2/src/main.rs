use d11p2::load;

fn main() {
    let mut solution = load("input.d11p1.full");
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer());
}
