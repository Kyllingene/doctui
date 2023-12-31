use cod::prelude::*;
use owo_colors::DynColor;
use rustdoc_types::{Crate, Id, ItemEnum};

use super::{module, theme};
use crate::links::{Link, Links};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemOrder {
    Module = 0,

    Primitive,
    Struct,
    Enum,
    Union,
    Trait,

    Function,
    Macro,

    Type,
    Constant,
    Static,
}

pub fn draw_name<'c>(
    cr: &'c Crate,
    id: &Id,
    links: &mut Links,
) -> Option<(ItemOrder, usize, impl FnOnce(u32, u32, usize) + 'c)> {
    let item = cr.index.get(id).expect("tried to get invalid item");

    use ItemOrder::*;
    let (kind, color, order) = match &item.inner {
        ItemEnum::Module(_) => ("mod", theme::MOD_NAME, Module),

        ItemEnum::Struct(_) => ("struct", theme::TYPE, Struct),
        ItemEnum::Enum(_) => ("enum", theme::TYPE, Enum),
        ItemEnum::Union(_) => ("union", theme::TYPE, Union),
        ItemEnum::Trait(_) => ("trait", theme::TYPE, Trait),

        ItemEnum::Function(_) => ("fn", theme::FN, Function),
        ItemEnum::Macro(_) => ("macro", theme::MACRO, Macro),

        ItemEnum::TypeAlias(_) => ("type", theme::TYPE, Type),
        ItemEnum::Constant(_) => ("const", theme::ITEM, Constant),
        ItemEnum::Static(_) => ("static", theme::ITEM, Static),

        ItemEnum::Primitive(_) => ("primitive", theme::ITEM, Primitive),

        _ => return None,
    };

    let name = item.name.as_ref().unwrap();

    let text = format!("\x1b[38;5;{color}m{name}\x1b[39m");
    let docline = item
        .docs
        .as_ref()
        .map(|doc| doc.lines().next().unwrap_or("").to_string());
    links.push(Link::new(text, id.clone(), order));

    Some((order, kind.len(), move |x, y, len| {
        goto::pos(x, y);
        print!("--- {kind:<len$} ");
        color::with::fg(color, || style::with::underline(|| print!("{name}")));

        if let Some(doc) = docline {
            color::with::fg(theme::DOC, || print!(" // {doc}"));
        }

        println!();
    }))
}

pub fn draw_full(cr: &Crate, id: &Id, x: u32, y: u32) -> (Links, u32) {
    let item = cr.index.get(id).expect("tried to get invalid item");
    let oy = y;

    (
        match &item.inner {
            ItemEnum::Module(_) => module::draw(cr, id, x, y),
            _ => Vec::new(),
        },
        y - oy,
    )
}
