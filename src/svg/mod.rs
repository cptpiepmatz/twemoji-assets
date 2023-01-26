pub mod codes;
pub mod names;

// TODO: check binary sizes
#[inline]
pub fn from_emoji(emoji: &str) -> Option<&'static str> {
    codes::from_emoji(emoji)
}

macro_rules! svg_code {
    ($ident:ident, $emoji:literal, $label:literal, $file_name:literal) => {
        #[doc = $emoji]
        #[doc = $label]
        pub const $ident: &str = include_str!(concat!(
            "../../assets/svg/",
            $file_name
        ));
    }
}

macro_rules! svg_name {
    ($alias:ident, $emoji:literal, $label:literal, $redirect:ident) => {
        #[doc = $emoji]
        #[doc = $label]
        pub const $alias: &str = $redirect;
    }
}

macro_rules! svg_match_emoji {
    [$(($emoji:literal, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static str> {
            match emoji {
                $($emoji => Some($ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use svg_code;
pub(crate) use svg_name;
pub(crate) use svg_match_emoji;
