use anyhow::Result;
use d25p1::load;
use log::{error, info};
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d25p1.full")?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse();
    match solution.answer() {
        Some(answer) => info!(
            "{}answer is {:?}",
            Paint::masked("🎅 "),
            Paint::bold(Paint::red(answer))
        ),
        _ => error!("{}No answer to the problem", Paint::masked("🎅 ")),
    }

    Ok(())
}
