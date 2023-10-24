use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub signature: Style,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Member {

}

pub fn parse_all(content: ElementRef<'_>) -> ParseResult<Vec<Impl>> {
    let hs = s!("h2");
    let ss = s!("div");
    let sections = content.select(&ss)
        .filter_map(|s| s.value().attr("id").map(|id| Some((s, AssociatedItemKind::parse(id)?))));

    let mut impls = Vec::new();
    for o in sections {
        let Some((section, kind)) = o else { continue; };
        if let Some(im) = parse_one(section, kind) {
            impls.push(im?);
        }
    }

    Ok(impls)
}

pub fn parse_one(im: ElementRef<'_>, kind: AssociatedItemKind) -> Option<ParseResult<Impl>> {
    type AIK = AssociatedItemKind;
    match kind {
        AIK::Method => Some(parse_methods(im)),
        AIK::AutoImplementation => Some(parse_autoimps(im)),
        AIK::Implementor => Some(parse_implementors(im)),
        AIK::TraitImplementation => Some(parse_traits(im)),
        AIK::BlanketImplementation => Some(parse_blanket(im)),
        AIK::RequiredMethod => Some(parse_reqmeth(im)),
        AIK::RequiredAssocType => Some(parse_reqtyp(im)),
        AIK::RequiredAssocConst => Some(parse_reqconst(im)),
        AIK::DerefMethod => Some(parse_deref(im)),
        AIK::ProvidedMethod => Some(parse_provided(im)),
        AIK::Variant => Some(parse_variants(im)),
        _ => None,
    }
}

fn parse_methods(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_autoimps(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_implementors(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_traits(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_blanket(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_reqmeth(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_reqtyp(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_reqconst(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_deref(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_provided(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
fn parse_variants(im: ElementRef<'_>) -> ParseResult<Impl> { todo!() }
