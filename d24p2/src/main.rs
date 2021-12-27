use anyhow::Result;
use d24p2::load;
use log::info;
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.d24p1.full")?;
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
