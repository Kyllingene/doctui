use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub signature: Style,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member {
    pub kind: AssociatedItemKind,
    pub name: Arc<str>,
    pub description: Option<Style>,
    pub definition: Option<Style>,
}

pub fn parse_all(content: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    let ss = s!("div");
    let sections = content.select(&ss).filter_map(|s| {
        s.value()
            .attr("id")
            .map(|id| Some((s, AssociatedItemKind::parse(id)?)))
    });

    let mut impls = Vec::new();
    for o in sections {
        let Some((section, kind)) = o else {
            continue;
        };
        if let Some(im) = parse_one(section, kind) {
            impls.append(&mut im?);
        }
    }

    Ok(impls)
}

pub fn parse_one(im: ElementRef<'_>, kind: AssociatedItemKind) -> Option<ParseResult<Vec<Impl>>> {
    type AIK = AssociatedItemKind;
    Some(match kind {
        AIK::Method => parse_methods(im),
        AIK::AutoImplementation => parse_autoimps(im),
        AIK::Implementor => parse_implementors(im),
        AIK::TraitImplementation => parse_traits(im),
        AIK::BlanketImplementation => parse_blanket(im),
        AIK::RequiredMethod => parse_reqmeth(im),
        AIK::RequiredAssocType => parse_reqtyp(im),
        AIK::RequiredAssocConst => parse_reqconst(im),
        AIK::DerefMethod => parse_deref(im),
        AIK::ProvidedMethod => parse_provided(im),
        AIK::Variant => parse_variants(im),
        _ => None?,
    })
}

fn parse_methods(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    let ds = s!(r#"details[class="toggle implementors-toggle"]"#);
    let impls_list = im.select(&ds);
    let mut impls = Vec::new();

    for im in impls_list {
        let signature = hierarchy!(
            im;
            "summary",
            r#"section[class="impl"]"#,
            "h3",
        )?;

        let signature = Style::parse(signature).ok_or_else(|| {
            err!(
                InvalidElement,
                "impl signature",
                Cow::Owned(signature.html())
            )
        })?;

        let mut members = Vec::new();
        let mut member_name = None;
        let mut member_kind = None;
        let mut member_description = None;
        let mut member_definition = None;
        let children = hierarchy!(im; r#"div[class="impl-items"]"#)?.children();
        for child in children {
            if let Some(child) = ElementRef::wrap(child) {
                match (child.value().name(), child.value().attr("class")) {
                    ("summary", _) => {
                        if let Some(name) = member_name.take() {
                            let kind = member_kind.take().unwrap_or(AssociatedItemKind::Method);
                            members.push(Member {
                                name,
                                kind,
                                description: member_description.take(),
                                definition: member_definition.take(),
                            });
                        }

                        let name = hierarchy!(
                            child;
                            r#"section[class="impl"]"#,
                            r#"h3[class="code-header"]"#,
                            "a",
                        )?
                        .text()
                        .collect::<String>()
                        .into();

                        member_name = Some(name);
                    }
                    // ("div", Some("docblock")) => {
                    //     member_description = Style::parse(child);
                    // }
                    ("details", Some("toggle method-toggle" | "toggle")) => {
                        if let Some(name) = member_name.take() {
                            let kind = member_kind.take().unwrap_or(AssociatedItemKind::Method);
                            members.push(Member {
                                name,
                                kind,
                                description: member_description.take(),
                                definition: member_definition.take(),
                            });
                        }

                        let def = hierarchy!(
                            child;
                            "summary",
                            r#"h4[class="code-header"]"#,
                        )?;

                        let name = hierarchy!(def; "a")?;
                        if let Some(mut args) = Style::parse(def) {
                            if args.len() > 1 {
                                args.drain(0..2);
                                member_definition = Some(args);
                            }
                        }

                        member_kind = name
                            .value()
                            .attr("class")
                            .map(|c| AssociatedItemKind::parse(c))
                            .flatten();
                        let name = name.text().collect::<String>().into();

                        member_name = Some(name);

                        let desc = hierarchy!(
                            child;
                            r#"div[class="docblock"]"#
                        )?;
                        member_description = Style::parse(desc);
                    }
                    ("section", Some("method")) => {
                        if let Some(name) = member_name.take() {
                            let kind = member_kind.take().unwrap_or(AssociatedItemKind::Method);
                            members.push(Member {
                                name,
                                kind,
                                description: member_description.take(),
                                definition: member_definition.take(),
                            });
                        }

                        let def = hierarchy!(
                            child;
                            r#"h4[class="code-header"]"#,
                        )?;

                        let name = hierarchy!(def; "a")?.text().collect::<String>().into();
                        member_name = Some(name);

                        if let Some(mut args) = Style::parse(def) {
                            if args.len() > 1 {
                                args.drain(0..2);
                                member_definition = Some(args);
                            }
                        }
                    }
                    _ => {
                        println!("warning: unparsed element: {}", child.html());
                    }
                }
            }
        }

        if let Some(name) = member_name.take() {
            let kind = member_kind.unwrap_or(AssociatedItemKind::Method);
            members.push(Member {
                name,
                kind,
                description: member_description.take(),
                definition: member_definition.take(),
            });
        }

        impls.push(Impl { signature, members });
    }

    Ok(impls)
}

fn parse_autoimps(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_implementors(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_traits(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_blanket(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_reqmeth(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_reqtyp(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_reqconst(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_deref(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_provided(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
fn parse_variants(im: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    Ok(Vec::new())
}
