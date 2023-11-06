#![allow(unused)]

pub const MODULE_ITEM_KINDS: [ModuleItemKind; ModuleItemKind::len()] = [
    ModuleItemKind::Module,
    ModuleItemKind::Keyword,
    ModuleItemKind::Struct,
    ModuleItemKind::Enum,
    ModuleItemKind::Union,
    ModuleItemKind::PrimitiveType,
    ModuleItemKind::Trait,
    ModuleItemKind::Macro,
    ModuleItemKind::AttributeMacro,
    ModuleItemKind::DeriveMacro,
    ModuleItemKind::Function,
    ModuleItemKind::TypeDefinition,
    ModuleItemKind::Constant,
];

/// An item associated with a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum ModuleItemKind {
    Module,
    Keyword,
    Struct,
    Enum,
    Union,
    PrimitiveType,
    Trait,
    Macro,
    AttributeMacro,
    DeriveMacro,
    Function,
    TypeDefinition,
    Constant,
}

impl ModuleItemKind {
    /// Parse a module item kind from either human-readable form
    /// (e.g. `Primitive Type`) or keyword form (e.g. `primitive`).
    pub fn parse(mut s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        for mut kind in s.split_whitespace() {
            if kind.ends_with('s') {
                kind = &kind[0..kind.len() - 1];
            }

            if let Some(kind) = Some(match kind {
                "module" | "mod" => Self::Module,
                "keyword" => Self::Keyword,
                "struct" => Self::Struct,
                "enum" => Self::Enum,
                "union" => Self::Union,
                "primitive type" | "primitive" => Self::PrimitiveType,
                "trait" => Self::Trait,
                "macro" => Self::Macro,
                "attribute macro" | "attr" => Self::AttributeMacro,
                "derive macro" | "derive" => Self::DeriveMacro,
                "function" | "fn" => Self::Function,
                "type definition" | "type" => Self::TypeDefinition,
                "constant" => Self::Constant,
                _ => None?,
            }) { return Some(kind); }
        }

        None
    }

    pub fn to_keyword(&self) -> &'static str {
        match self {
            Self::Module => "mod",
            Self::Keyword => "keyword",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Union => "union",
            Self::PrimitiveType => "primitive",
            Self::Trait => "trait",
            Self::Macro => "macro",
            Self::AttributeMacro => "attr",
            Self::DeriveMacro => "derive",
            Self::Function => "fn",
            Self::TypeDefinition => "type",
            Self::Constant => "constant",
        }
    }

    pub fn to_human(&self) -> &'static str {
        match self {
            Self::Module => "Module",
            Self::Keyword => "Keyword",
            Self::Struct => "Struct",
            Self::Enum => "Enum",
            Self::Union => "Union",
            Self::PrimitiveType => "Primitive Type",
            Self::Trait => "Trait",
            Self::Macro => "Macro",
            Self::AttributeMacro => "Attribute Macro",
            Self::DeriveMacro => "Derive Macro",
            Self::Function => "Function",
            Self::TypeDefinition => "Type Definition",
            Self::Constant => "Constant",
        }
    }

    /// The number of module item kinds.
    pub const fn len() -> usize {
        Self::Constant as usize + 1
    }
}

pub const ASSOCIATED_ITEM_KINDS: [AssociatedItemKind; AssociatedItemKind::len()] = [
    AssociatedItemKind::Constant,
    AssociatedItemKind::Field,
    AssociatedItemKind::Declaration,
    AssociatedItemKind::ProvidedAssocConst,
    AssociatedItemKind::Variant,
    AssociatedItemKind::Method,
    AssociatedItemKind::AutoImplementation,
    AssociatedItemKind::RequiredMethod,
    AssociatedItemKind::RequiredAssocType,
    AssociatedItemKind::RequiredAssocConst,
    AssociatedItemKind::ProvidedMethod,
    AssociatedItemKind::Implementor,
    AssociatedItemKind::TraitImplementation,
    AssociatedItemKind::BlanketImplementation,
    AssociatedItemKind::AssocConstant,
    AssociatedItemKind::DerefMethod,
];

/// An item associated with a struct, enum, or trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum AssociatedItemKind {
    Constant,
    Field,
    Declaration,
    ProvidedAssocConst,
    Variant,
    Method,
    AutoImplementation,
    RequiredMethod,
    RequiredAssocType,
    RequiredAssocConst,
    ProvidedMethod,
    Implementor,
    TraitImplementation,
    BlanketImplementation,
    AssocConstant,
    DerefMethod,
}

impl AssociatedItemKind {
    pub fn parse(mut s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        for mut kind in s.split_whitespace() {
            if kind.ends_with('s') {
                kind = &kind[0..kind.len() - 1];
            }

            if let Some(kind) = Some(match kind {
                "const" | "constant" => Self::Constant,
                "field" | "structfield" => Self::Field,
                "declaration" | "item-decl" | "decl" => Self::Declaration,
                "provided-associated-constant" | "provided-associated-const" => {
                    Self::ProvidedAssocConst
                }
                "variant" => Self::Variant,
                "method" | "implementation" | "implementations-list" => Self::Method,
                "auto trait implementation"
                | "synthetic-implementation"
                | "synthetic-implementations-list" => Self::AutoImplementation,
                "required-method" | "required-methods-list" => Self::RequiredMethod,
                "required-associated-type" | "required-associated-types-list" => {
                    Self::RequiredAssocType
                }
                "required-associated-constant" | "required-associated-consts-list" => {
                    Self::RequiredAssocConst
                }
                "provided-method" | "provided-methods-list" => Self::ProvidedMethod,
                "implementor" | "implementors-list" => Self::Implementor,
                "trait-implementation" | "trait-implementations-list" => Self::TraitImplementation,
                "blanket-implementation" | "blanket-implementations-list" => {
                    Self::BlanketImplementation
                }
                _ => None?,
            }) { return Some(kind); }
        }

        None
    }

    pub fn to_keyword(&self) -> &'static str {
        match self {
            Self::Constant => "const",
            Self::Field => "field",
            Self::Declaration => "item-decl",
            Self::ProvidedAssocConst => "provided-assoc-const",
            Self::Variant => "variant",
            Self::Method => "implementation",
            Self::AutoImplementation => "synthetic-implementation",
            Self::RequiredMethod => "required-method",
            Self::RequiredAssocType => "required-associated-type",
            Self::RequiredAssocConst => "required-associated-const",
            Self::ProvidedMethod => "provided-method",
            Self::Implementor => "implementor",
            Self::TraitImplementation => "trait-implementation",
            Self::BlanketImplementation => "blanket-implementation",
            Self::AssocConstant => "associatedconstant",
            Self::DerefMethod => "deref-method",
        }
    }

    pub fn to_human(&self) -> &'static str {
        match self {
            Self::Constant => "Constant",
            Self::Field => "Field",
            Self::Declaration => "Item Declaration",
            Self::ProvidedAssocConst => "Provided Associated Constant",
            Self::Variant => "Variant",
            Self::Method => "Method",
            Self::AutoImplementation => "Auto Trait Implementation",
            Self::RequiredMethod => "Required Method",
            Self::RequiredAssocType => "Required Associated Type",
            Self::RequiredAssocConst => "Required Associated Constant",
            Self::ProvidedMethod => "Provided Method",
            Self::Implementor => "Implementor",
            Self::TraitImplementation => "Trait Implementation",
            Self::BlanketImplementation => "Blanket Implementation",
            Self::AssocConstant => "Associated Constant",
            Self::DerefMethod => "Methods from",
        }
    }

    /// The number of associated item kinds.
    pub const fn len() -> usize {
        Self::DerefMethod as usize + 1
    }
}
