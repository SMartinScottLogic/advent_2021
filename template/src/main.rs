use template::load;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.full")?;
    println!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {}", solution.answer()?);

    Ok(())
}
