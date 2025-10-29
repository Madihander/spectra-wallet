use anyhow::Result;
mod cli;

fn main() -> Result<()> {
    let _ = cli::run_cli();

    Ok(())

}