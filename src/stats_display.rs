use crate::file_analizer::Stats;
use crate::types::*;

pub fn print(stats: Stats) -> ProjResult<()> {
    println!("{:?}", stats);
    Ok(())
}
