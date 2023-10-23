#[macro_export]
macro_rules! hierarchy {
    ( $page:expr ; $selector1:expr $( , $selector:expr )* $(,)? ) => {(|| {
        let data = $page.select(
            &scraper::Selector::parse($selector1).expect(&format!("failed to parse {}", $selector1))
        ).next().ok_or_else(|| crate::parse::ParseError::ElementNotFound($selector1, stringify!($page)))?;
        let _prev = $selector1;

        $(
            let data = data.select(
                &scraper::Selector::parse($selector).expect(&format!("failed to parse {}", $selector))
            ).next().ok_or_else(|| crate::parse::ParseError::ElementNotFound($selector, _prev))?;
            let _prev = $selector;
        )*

        Ok(data)
    })()}
}

#[macro_export]
macro_rules! s {
    ( $selector:expr ) => {
        scraper::Selector::parse($selector).expect(&format!("failed to parse {}", $selector))
    };
}

#[macro_export]
macro_rules! err {
    ( $type:ident, $( $arg:expr ),* $(,)? ) => {
        crate::parse::ParseError::$type($($arg,)*)
    }
}
