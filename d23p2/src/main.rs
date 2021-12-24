use std::env;

use anyhow::Result;
use d23p2::load;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();

    let filename = env::args().nth(1).unwrap();
    let mut solution = load(&filename)?;
    debug!("solution: {:?}", solution);
    solution.analyse();
    println!("answer is {:?}", solution.answer());

    Ok(())
}
