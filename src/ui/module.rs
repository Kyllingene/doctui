use antsy::AnsiStr;
use cod::prelude::*;
use rustdoc_types::{Crate, Id, ItemEnum};

use crate::ui::deprecation;

use super::{item, theme};
use crate::links::Links;

pub fn draw(cr: &Crate, id: &Id, x: u32, mut y: u32) -> Links {
    let root = cr.index.get(id).unwrap();
    let ItemEnum::Module(inner) = &root.inner else {
        panic!("called module::draw on non-module item");
    };

    goto::pos(x, y);
    y += 2;

    if inner.is_crate {
        print!(" crate ");
    } else {
        print!(" mod ");
    }

    style::with::bold(|| {
        color::with::fg(theme::MOD_NAME, || {
            println!("{}", root.name.as_ref().unwrap())
        })
    });

    let mut len = root.name.as_ref().unwrap().len() + 8;
    if let Some(dep) = &root.deprecation {
        let s = deprecation(dep);
        len = len.max(AnsiStr::new(&s).len() + 2);
        println!(" {s}");

        y += s.lines().count() as u32;
    }

    if let Some(doc) = &root.docs {
        color::with::fg(theme::DOC, || {
            println!();
            y += 1;
            for line in doc.lines() {
                println!("| {line}");
                y += 1;
            }
        });
    }

    style::with::faint(|| println!("{}", "=".repeat(len)));

    let mut links = Links::new();
    let mut items = Vec::with_capacity(inner.items.len());
    for id in &inner.items {
        if let Some(item) = item::draw_name(cr, id, &mut links) {
            items.push(item);
        }
    }

    let len = items.iter().map(|(_, l, _)| *l).max();

    items.sort_unstable_by_key(|(o, _, _)| *o);
    links.sort_unstable();
    items.into_iter().for_each(|(_, _, f)| {
        f(x, y, len.unwrap());
        y += 1;
    });

    links
}
