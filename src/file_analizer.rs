use crate::arg_parser::Args;
use crate::language_file_extensions::LANGS;
use crate::types::*;

pub struct Stats {}

pub fn run(args: Args, files: Files) -> ProjResult<Stats> {
    println!("{:?}", LANGS.get(".h"));
    todo!();
}
