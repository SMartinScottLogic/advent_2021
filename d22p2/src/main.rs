use anyhow::Result;
use d22p2::load;
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d22p1.full")?;
    debug!("solution: {:?}", solution);
    solution.analyse();
    info!("answer is {}", solution.answer()?);

    Ok(())
}
