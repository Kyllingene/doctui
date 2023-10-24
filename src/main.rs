use errata::FallibleExt;
use sarge::prelude::*;
use scraper::Html;

mod helper;
mod item;
mod link;
mod parse;
mod prelude;

pub type Str = std::borrow::Cow<'static, str>;

// TODO: properly parse all sub-HTML such as bold, links, etc.

#[errata::catch]
fn main() {
    let parser = ArgumentParser::new();

    let files = parser.parse().fail("failed to parse arguments");

    let file = files.get(0).fail("you must specify a file");
    let data = std::fs::read_to_string(&file).fail("failed to read file");
    let html = Html::parse_document(&data);

    let page = parse::parse(&html).fail("failed to parse file");

    if let parse::Parsed::CrateItem(page) = page {
        println!("=== {} {} ===", page.kind().to_human(), page.name());
        if let Some(d) = page.description() {
            println!("{}", d.normal());
        }

        println!("---");
    } else if let parse::Parsed::CratePage(page) = page {
        println!("=== Crate {} ===", page.name);
        println!("Version {}", page.version);
        println!("---\n");

        if let Some(d) = page.description {
            println!("{}", d.normal());
        }

        println!("---\n");
    }
}
