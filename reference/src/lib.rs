#![deprecated(since = "0.1.1", note = "Deprecation notice")]
//! Crate-level documentation.

pub type TypeAlias = u8;
pub const CONSTANT: TypeAlias = 0xdeadbeef;
pub static STATIC: i32 = CONSTANT as i32;

/// module docs
///
/// Second line of module docs
pub mod module {
    /// F docs
    pub trait F {
        /// F::B docs
        type B;
        /// F::C docs
        const C: u8;

        /// F::DEF docs
        const DEF: u8 = 123;

        /// F::foobar docs
        fn foobar() {}
    }
}

/// T docs
pub struct T {
    /// T.asdf docs
    pub asdf: u8,
    pub ghjk: f32,
}

/// impl T (1) docs
impl T {
    /// T::D docs
    pub const D: u8 = 2;

    /// T::somefun docs
    pub fn somefun(a: u8) {
        drop(a)
    }
    pub fn otherfun() {}
}

impl T {
    pub fn yetanother() {}
}

/// impl F for T docs
impl module::F for T {
    /// T(F)::B docs
    type B = u8;
    /// T(F)::C docs
    const C: u8 = 1;
}

/// Z docs
union Z {
    /// Z.x docs
    pub x: f32,
    /// Z.y docs
    pub y: i32,
}

/// Q docs
pub enum Q {
    /// Q::A docs
    A,
    /// Q::B docs
    B(u8, f32),
    /// Q::C docs
    C { d: u8, e: f32 },
}

/// G docs
pub struct G<F>(F);

impl<F> G<F> {
    pub fn foo(f: F) -> F {
        f
    }
}

/// decl_macro docs
#[macro_export]
macro_rules! decl_macro {
    ( $foo:expr ) => {};
    ( $bar:expr ) => {};
}

/// root_fn docs
pub fn root_fn(f: u8, g: u8) {
    drop(f);
    drop(g);
}
