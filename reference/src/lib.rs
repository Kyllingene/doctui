type HAHAHAHAHAHAHA = u8;
const IHATEMYLIFE: HAHAHAHAHAHAHA = 0xdeadbeef;

/// F docs
trait F {
    /// F::B docs
    type B;
    /// F::C docs
    const C: u8;

    /// F::DEF docs
    const DEF: u8 = 123;

    /// F::foobar docs
    fn foobar() {}
}

/// T docs
struct T {
    /// T.asdf docs
    asdf: u8,
    ghjk: f32,
}

/// impl T (1) docs
impl T {
    /// T::D docs
    const D: u8 = 2;

    /// T::somefun docs
    fn somefun(a: u8) { drop(a) }
    fn otherfun() {}
}

impl T {
    fn yetanother() {}
}

/// impl F for T docs
impl F for T {
    /// T(F)::B docs
    type B = u8;
    /// T(F)::C docs
    const C: u8 = 1;
}

/// Z docs
union Z {
    /// Z.x docs
    x: f32,
    /// Z.y docs
    y: i32,
}

/// Q docs
enum Q {
    /// Q::A docs
    A,
    /// Q::B docs
    B(u8, f32),
    /// Q::C docs
    C { d: u8, e: f32 }
}

/// haha docs
macro_rules! haha {
    ( $foo:expr ) => {};
    ( $bar:expr ) => {};
}

/// foobarfoobar docs
pub fn foobarfoobar(
    f: u8,
    g: u8
) { drop(f); drop(g); }

