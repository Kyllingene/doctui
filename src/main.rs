use errata::{FallibleExt, error};
use sarge::prelude::*;
use dirs::data_dir;

mod backends;
pub use backends::parse; // TODO: refactor this out
mod helper;
mod item;
mod link;
mod prelude;
mod style;

pub type Str = std::borrow::Cow<'static, str>;

#[errata::catch]
fn main() {
    let parser = ArgumentParser::new();
    let backend = parser.add(tag::both('b', "backend"));

    let files = parser.parse().fail("failed to parse arguments");
    let file = files.get(0).fail("you must specify a file");
    
    let mut data_dir = data_dir().unwrap_or_else(|| "./doctui".into());
    data_dir.push("doctui");
    std::fs::create_dir_all(&data_dir).fail("failed to create data directory");

    let docs_path = match backend.get().unwrap_or_else(|_| "json".to_string()).as_str() {
        "html" => error!("parsing backend not yet complete"),
        "json" => {
            rustdoc_json::Builder::default()
                .toolchain("nightly")
                .target_dir(&data_dir)
                .all_features(true)
                .manifest_path(file)
                .build()
                .fail("failed to generate JSON documentation")
        }
        back => error!("invalid backend: `{back}`"),
    };

    println!("{}", docs_path.display());
}
