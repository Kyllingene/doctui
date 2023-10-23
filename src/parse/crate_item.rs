use std::sync::Arc;
use std::borrow::Cow;

use scraper::Html;

use super::ParseResult;
use crate::item::AssociatedItemKind;

pub type Sections = [Vec<AssociatedItem>; AssociatedItemKind::len()];

pub enum AssociatedItem {
    Method {
        signature: Arc<str>,
    },
}

pub struct CrateItem {
    pub name: Arc<str>,
    pub kind: Arc<str>,
    pub description: Arc<str>,

    pub sections: Sections,
    // stable: Option<Arc<str>>, TODO: should this be included?
}

pub fn parse(page: &Html) -> ParseResult<CrateItem> {
    todo!()
}
