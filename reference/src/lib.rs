type HAHAHAHAHAHAHA = u8;
const IHATEMYLIFE: HAHAHAHAHAHAHA = 0xdeadbeef;

trait F {
    type B;
    const C: u8;

    const DEF: u8 = 123;

    fn foobar() {}
}

struct T {
    /// DOCUMENTATION
    asdf: u8,
    ghjk: f32,
}

impl T {
    const D: u8 = 2;

    fn somefun() {}
}

impl F for T {
    type B = u8;
    const C: u8 = 1;
}

union Z {
    x: f32,
    y: i32,
}

enum Q {
    A,
    B(u8, f32),
    C { d: u8, e: f32 }
}

macro_rules! haha {
    ( $foo:expr ) => {};
    ( $bar:expr ) => {};
}

