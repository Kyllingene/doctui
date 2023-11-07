use std::sync::Arc;

use scraper::Html;

use crate::style::Style;
use crate::parse::ParseResult;
use crate::{hierarchy, maybe};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyword {
    pub name: Arc<str>,
    pub description: Option<Style>,
}

pub fn parse(page: &Html) -> ParseResult<Keyword> {
    let content = hierarchy!(
        page;
        "body",
        "main",
        r#"div[class="width-limiter"]"#,
        r#"section[id="main-content"]"#,
    )?;

    let name = hierarchy!(
        content;
        r#"div[class="main-heading"]"#,
        "h1",
        r#"a[class="keyword"]"#,
    )?
    .text()
    .collect::<String>()
    .into();

    let description = maybe!(
        content;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#,
    )
    .map(|desc| Style::parse(desc))
    .flatten();

    Ok(Keyword { name, description })
}
