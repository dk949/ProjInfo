use crate::file_analizer::Stats;
use crate::types::*;

pub fn print(stats: Stats) -> ProjResult<()> {
    const MAX_WITH: usize = 50;
    const FILL: &str = "─";
    const SPACE: &str = " ";
    let title = " Summary ";
    let total_count = stats.total.to_string();
    println!("╭─{title:─<titleWidth$}─╮", titleWidth = MAX_WITH);
    for (language, count) in stats.langs.iter() {
        let percentage = format!("{:.2}%", ((*count as f32) / (stats.total as f32)) * 100.);
        println!(
            "│ {language}: {percentage: <langWidth$}│",
            langWidth = MAX_WITH - (language.len()) - 1
        );
    }

    println!("│ {SPACE: <spacerWidth$} │", spacerWidth = MAX_WITH);
    println!(
        "│ Total files: {total_count: <langWidth$}│",
        langWidth = MAX_WITH - ("Total files").len() - 1
    );
    println!("╰─{FILL:─<footerWidth$}─╯", footerWidth = MAX_WITH);

    Ok(())
}
