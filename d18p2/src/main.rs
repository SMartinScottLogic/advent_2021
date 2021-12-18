use anyhow::Result;
use d18p2::load;
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d18p1.full")?;
    debug!("solution: {:?}", solution);
    solution.analyse();
    info!("answer is {}", solution.answer()?);

    Ok(())
}
