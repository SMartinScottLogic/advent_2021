use anyhow::Result;
use d23p1::load;
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d23p1.full")?;
    debug!("solution: {:?}", solution);
    solution.analyse();
    info!("answer is {:?}", solution.answer());

    Ok(())
}
