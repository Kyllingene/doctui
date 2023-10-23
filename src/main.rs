use errata::FallibleExt;
use sarge::prelude::*;
use scraper::Html;

mod helper;
mod item;
mod link;
mod parse;

// TODO: properly parse all sub-HTML such as bold, links, etc.

#[errata::catch]
fn main() {
    let parser = ArgumentParser::new();

    let files = parser.parse().fail("failed to parse arguments");

    let file = files.get(0).fail("you must specify a file");
    let data = std::fs::read_to_string(&file).fail("failed to read file");
    let html = Html::parse_document(&data);

    let page = parse::crate_page::parse(&html).fail("failed to parse file");
    println!("=== Crate {} (version {})", page.name, page.version);
    println!("---\n{}\n---", &page.description[0..100]);

    for (i, section) in page.sections.iter().enumerate() {
        let cap = 3.min(section.len());
        if cap != 0 {
            println!("= {:?} {}", item::MODULE_ITEM_KINDS[i], section.len());
            for item in &section[0..cap] {
                println!("{item:#?}");
            }
        }
    }
}
