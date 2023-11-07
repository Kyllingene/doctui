use std::path::Path;

pub mod parse;

pub fn parse<P: AsRef<Path>>(crate_path: P) -> parse::ParseResult<parse::Parsed> {
    todo!()
}
