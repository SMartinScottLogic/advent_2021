use anyhow::Result;
use log::info;
use template::load;
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.full")?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse();
    info!(
        "{}answer is {}",
        Paint::masked("🎅 "),
        Paint::bold(Paint::red(solution.answer()?))
    );

    Ok(())
}
