#![allow(unused)]

macro_rules! def_color {
    ( $( $name:ident = $val:expr ),+ $(,)? ) => {
        $(
            pub const $name: u8 = $val;
        )+
    };
}

def_color! {
    BLACK = 0,
    RED = 1,
    GREEN = 2,
    YELLOW = 3,
    BLUE = 4,
    PURPLE = 5,
    CYAN = 6,
    LGRAY = 7,
    GRAY = 8,
    LRED = 9,
    LGREEN = 10,
    LYELLOW = 11,
    LBLUE = 12,
    PINK = 13,
    LCYAN = 14,
    WHITE = 15,

    ACCENT = LGREEN,

    MOD_NAME = CYAN,
    TYPE = YELLOW,
    ITEM = BLUE,
    FN = 166,
    MACRO = PURPLE,
    DOC = 245,
}
