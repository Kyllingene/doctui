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
        if s.ends_with('s') {
            s = &s[0..s.len() - 1];
        }

        let s = s.to_lowercase();

        Some(match s.as_str() {
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
        })
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
    AssociatedItemKind::Method,
    AssociatedItemKind::AutoImplementation,
    AssociatedItemKind::RequiredMethod,
    AssociatedItemKind::RequiredAssocType,
    AssociatedItemKind::RequiredAssocConst,
    AssociatedItemKind::ProvidedMethod,
    AssociatedItemKind::Implementor,
    AssociatedItemKind::TraitImplementation,
    AssociatedItemKind::BlanketImplementation,
    AssociatedItemKind::DerefMethod,
];

/// An item associated with a struct, enum, or trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum AssociatedItemKind {
    Method,
    AutoImplementation,
    RequiredMethod,
    RequiredAssocType,
    RequiredAssocConst,
    ProvidedMethod,
    Implementor,
    TraitImplementation,
    BlanketImplementation,
    DerefMethod,
}

impl AssociatedItemKind {
    pub fn parse(mut s: &str) -> Option<Self> {
        if s.ends_with('s') {
            s = &s[0..s.len() - 1];
        }

        let s = s.to_lowercase().replace(' ', "-");

        if s.starts_with("methods-from") || s.starts_with("deref-methods-") {
            return Some(Self::DerefMethod);
        }

        Some(match s.as_str() {
            "method" | "implementation" => Self::Method,
            "auto trait implementation" | "synthetic-implementation" => Self::AutoImplementation,
            "required-method" => Self::RequiredMethod,
            "required-associated-types" => Self::RequiredAssocType,
            "required-associated-constants" => Self::RequiredAssocConst,
            "provided-methods" => Self::ProvidedMethod,
            "implementors" => Self::Implementor,
            "trait-implementation" => Self::TraitImplementation,
            "blanket-implementation" => Self::BlanketImplementation,
            _ => None?,
        })
    }

    /// The number of associated item kinds.
    pub const fn len() -> usize {
        Self::DerefMethod as usize + 1
    }
}
