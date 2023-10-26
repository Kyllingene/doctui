type HAHAHAHAHAHAHA = u8;
const IHATEMYLIFE: HAHAHAHAHAHAHA = 0xdeadbeef;

/// DOCUMENTATION 18
trait F {
    /// DOCUMENTATION 19
    type B;
    /// DOCUMENTATION 20
    const C: u8;

    /// DOCUMENTATION 21
    const DEF: u8 = 123;

    /// DOCUMENTATION 22
    fn foobar() {}
}

/// DOCUMENTATION 1
struct T {
    /// DOCUMENTATION 2
    asdf: u8,
    ghjk: f32,
}

/// DOCUMENTATION 3
impl T {
    /// DOCUMENTATION 4
    const D: u8 = 2;

    /// DOCUMENTATION 5
    fn somefun() {}
    fn otherfun() {}
}

impl T {
    fn yetanother() {}
}

/// DOCUMENTATION 6
impl F for T {
    /// DOCUMENTATION 7
    type B = u8;
    /// DOCUMENTATION 8
    const C: u8 = 1;
}

/// DOCUMENTATION 9
union Z {
    /// DOCUMENTATION 10
    x: f32,
    /// DOCUMENTATION 11
    y: i32,
}

/// DOCUMENTATION 12
enum Q {
    /// DOCUMENTATION 13
    A,
    /// DOCUMENTATION 14
    B(u8, f32),
    /// DOCUMENTATION 15
    C { d: u8, e: f32 }
}

/// DOCUMENTATION 16
macro_rules! haha {
    ( $foo:expr ) => {};
    ( $bar:expr ) => {};
}

/// DOCUMENTATION 17
pub fn foobarfoobar(
    f: u8,
    g: u8
) { drop(f); drop(g); }

