use std::sync::Arc;

use scraper::Html;

use super::ParseResult;
use crate::style::Style;
use crate::{hierarchy, maybe};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant {
    pub name: Arc<str>,
    pub definition: Style,
    pub description: Option<Style>,
}

pub fn parse(page: &Html) -> ParseResult<Constant> {
    let body = hierarchy!(page; r#"body"#)?;

    let main = hierarchy!(
        body;
        "main",
        r#"div[class="width-limiter"]"#,
        r#"section[id="main-content"]"#,
    )?;

    let name = hierarchy!(
        main;
        "h1",
        r##"a[href="#"]"##
    )?
    .text()
    .collect::<String>()
    .into();

    let description = maybe!(
        main;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#
    )
    .map(|desc| Style::parse(desc))
    .flatten();

    let definition = Style::parse(hierarchy!(
        main;
        r#"pre[class="rust item-decl"]"#,
        "code"
    )?)
    .unwrap_or_else(Style::new);

    Ok(Constant {
        name,
        definition,
        description,
    })
}
