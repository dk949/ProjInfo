use crate::arg_parser::Args;
use crate::file_analizer::Stats;
use crate::types::*;

pub fn print(args: &Args, stats: Stats) -> ProjResult<()> {
    const MAX_WITH: usize = 50;
    const FILL: &str = "─";
    const SPACE: &str = " ";
    let title = " Summary ";
    let total_count = stats.total.to_string();
    let mut langs: Vec<_> = stats.langs.iter().collect::<Vec<_>>();
    langs.sort_by(|a, b| a.1.cmp(b.1).reverse());
    let mut num_shown = 0;
    let mut other_count = 0;
    let mut unknown = (false, String::new());

    println!("╭─{title:─<titleWidth$}─╮", titleWidth = MAX_WITH);
    for (language, count) in langs {
        if num_shown > args.most {
            other_count += count;
        } else {
            let percentage = format!("{:.2}%", ((*count as f32) / (stats.total as f32)) * 100.);
            if language == "Unknown" {
                unknown = (
                    true,
                    format!(
                        "│ {language}: {percentage: <langWidth$}│",
                        langWidth = MAX_WITH - (language.len()) - 1
                    ),
                );
            } else {
                println!(
                    "│ {language}: {percentage: <langWidth$}│",
                    langWidth = MAX_WITH - (language.len()) - 1
                );
            }
        }
        num_shown += 1;
    }

    if other_count > 0 {
        let percentage = format!("{:.2}%", ((other_count as f32) / (stats.total as f32)) * 100.);
        println!(
            "│ Other: {percentage: <width$}│",
            width = MAX_WITH - ("Other").len() - 1
        );
    }

    if unknown.0 {
        println!("{}", unknown.1);
    }

    println!("│ {SPACE: <spacerWidth$} │", spacerWidth = MAX_WITH);
    println!(
        "│ Total files: {total_count: <langWidth$}│",
        langWidth = MAX_WITH - ("Total files").len() - 1
    );
    println!("╰─{FILL:─<footerWidth$}─╯", footerWidth = MAX_WITH);

    Ok(())
}
