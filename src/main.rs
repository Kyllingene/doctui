#![allow(unused_imports)]
use errata::{error, FallibleExt};
use sarge::prelude::*;
use serde_json::from_str;

mod links;
mod ui;

sarge! {
    Args,

    #ok 'f' file: bool,
}

#[errata::catch]
fn main() {
    #[allow(unused)]
    let (_args, files) = Args::parse().fail("failed to parse arguments");
    let file = files.get(1).fail("you must specify a crate");

    let mut data_dir = dirs::data_dir().unwrap_or_else(|| "./doctui".into());
    data_dir.push("doctui");
    std::fs::create_dir_all(&data_dir).fail("failed to create data directory");

    let docs_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .target_dir(&data_dir)
        .all_features(true)
        .manifest_path(file)
        .build()
        .fail("failed to generate JSON documentation");

    let data = std::fs::read_to_string(docs_path).fail("failed to read JSON documentation");
    let cr = from_str(&data).fail("failed to parse JSON documentation");

    ui::run(&cr);
}
