use anyhow::Result;
use d24p1::load;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d24p1.full")?;
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer()?);

    Ok(())
}
