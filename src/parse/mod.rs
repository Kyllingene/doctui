use std::borrow::Cow;

use scraper::Html;
use thiserror::Error;

pub mod all_page;
pub mod crate_page;
pub mod crate_item;

pub type ParseResult<T> = Result<T, ParseError>;

pub enum Parsed {
    AllPage(all_page::Sections),
    CratePage(crate_page::Crate),
    CrateItem(crate_item::CrateItem),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("failed to find {0} in {1}")]
    ElementNotFound(&'static str, &'static str),

    #[error("found invalid {0}: {1}")]
    InvalidElement(&'static str, Cow<'static, str>),
}

pub fn parse(page: &Html) -> ParseResult<Parsed> {
    if let Ok(ap) = all_page::parse(page) {
        Ok(Parsed::AllPage(ap))
    } else if let Ok(cp) = crate_page::parse(page) {
        Ok(Parsed::CratePage(cp))
    } else {
        crate_item::parse(page).map(|ci| Parsed::CrateItem(ci))
    }
}
