use antsy::AnsiStr;
use cod::console;
use cod::prelude::*;
use cod::Key;
use errata::FallibleExt;
use owo_colors::OwoColorize;
use rustdoc_types::{Crate, Deprecation};

use crate::links::Link;

pub mod item;
mod module;
mod theme;

pub fn run(cr: &Crate) {
    let _guard = guard::Reset;
    clear::all();

    let mut history = vec![cr.root.clone()];
    let mut history_point = 0;

    let mut links = module::draw(cr, &cr.root, 0, 0);
    draw_links(&links);

    let mut current = 0;
    loop {
        if let Some(link) = links.get(current) {
            let len = AnsiStr::new(&link.text).len();
            goto::pos(u32::MAX - 1, current as u32 + 3);
            goto::left(len as u32 + 2);
            color::with::fg(theme::ACCENT, || print!(">"));
            cod::goto::left(1);
            cod::flush();
        }

        let Some(key) = read::key() else { continue };

        match key {
            Key::Char('q') => break,
            Key::ArrowUp | Key::Char('k') => {
                if current > 0 {
                    current -= 1;
                    print!(" ");
                }
            }
            Key::ArrowDown | Key::Char('j') => {
                if current < links.len().saturating_sub(1) {
                    current += 1;
                    print!(" ");
                }
            }
            Key::ArrowLeft | Key::Char('h') => {
                if history_point > 0 {
                    history_point -= 1;
                    let new_id = &history[history_point];

                    clear::all();
                    current = 0;
                    links = item::draw_full(cr, new_id, 0, 0).0;
                    draw_links(&links);
                }
            }
            Key::ArrowRight | Key::Char('l') => {
                if history_point < history.len().saturating_sub(1) {
                    history_point += 1;
                    let new_id = &history[history_point];

                    clear::all();
                    current = 0;
                    links = item::draw_full(cr, new_id, 0, 0).0;
                    draw_links(&links);
                }
            }
            Key::Char(' ') => {
                if let Some(link) = links.get(current) {
                    let new_id = link.id.clone();

                    clear::all();
                    current = 0;
                    links = item::draw_full(cr, &new_id, 0, 0).0;
                    draw_links(&links);

                    history_point += 1;
                    history.resize_with(history_point, || unreachable!());
                    history.push(new_id);
                }
            }
            _ => {}
        }
    }

    clear::all();
    goto::home();
    cod::flush();
}

fn draw_links(links: &[Link]) {
    goto::pos(u32::MAX - 1, 3);
    for link in links {
        let len = AnsiStr::new(&link.text).len();
        goto::left(len as u32 + 3);
        println!("   {}", link.text);
        goto::right(u32::MAX - 1);
    }
}

fn deprecation(dep: &Deprecation) -> String {
    let mut buf = "Deprecated".bold().red().to_string();

    if let Some(since) = dep.since.as_ref() {
        buf.push_str(" since ");
        buf.push_str(&since.yellow().to_string());
    }

    if let Some(note) = dep.note.as_ref() {
        buf.push_str(": ");
        buf.push_str(note);
    }

    buf
}
