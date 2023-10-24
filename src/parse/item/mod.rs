use std::sync::Arc;

use scraper::Html;

use super::style::Style;
use super::ParseResult;
use crate::item::ModuleItemKind;

mod constant;
mod keyword;
mod r#struct;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CrateItem {
    Constant(constant::Constant),
    Keyword(keyword::Keyword),
    Struct(r#struct::Struct),
}

impl CrateItem {
    pub fn kind(&self) -> ModuleItemKind {
        match self {
            Self::Constant(_) => ModuleItemKind::Constant,
            Self::Keyword(_) => ModuleItemKind::Keyword,
            Self::Struct(_) => ModuleItemKind::Struct,
        }
    }

    pub fn name(&self) -> Arc<str> {
        match self {
            Self::Constant(c) => c.name.clone(),
            Self::Keyword(k) => k.name.clone(),
            Self::Struct(s) => s.name.clone(),
        }
    }

    pub fn description(&self) -> Option<&Style> {
        match self {
            Self::Constant(c) => c.description.as_ref(),
            Self::Keyword(k) => k.description.as_ref(),
            Self::Struct(s) => s.description.as_ref(),
        }
    }
}

pub fn parse(page: &Html) -> ParseResult<CrateItem> {
    if let Ok(c) = constant::parse(&page) {
        Ok(CrateItem::Constant(c))
    } else if let Ok(k) = keyword::parse(&page) {
        Ok(CrateItem::Keyword(k))
    } else if let Ok(s) = r#struct::parse(&page) {
        Ok(CrateItem::Struct(s))
    } else {
        todo!()
    }
}
