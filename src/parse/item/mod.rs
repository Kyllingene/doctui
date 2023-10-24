use scraper::Html;

use super::ParseResult;

mod constant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CrateItem {
    Constant(constant::Constant),
}

pub fn parse(page: &Html) -> ParseResult<CrateItem> {
    constant::parse(&page).map(|c| CrateItem::Constant(c))
}

