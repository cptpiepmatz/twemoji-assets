//! SVG Twemoji assets.
//!
//! This module provides access to the Twemoji assets in SVG format.
//! All emojis are sourced from the
//! [Twemoji repository](https://github.com/twitter/twemoji/tree/master/assets/svg).
//!
//! The module has two main components:
//! - The [`codes`](codes) module, which provides all assets named by their unicode characters.
//! - The [`names`](codes) module, which provides a subset of emojis with human-readable names.
//!   This module requires the `names` feature to be enabled.
//!
//! The main type to use when interacting with the module is the
//! [`SvgTwemojiAsset`](SvgTwemojiAsset) type definition.
//! It provides convenient methods for loading assets from either a string containing the emoji's
//! unicode character (e.g. `"🦆"`) or the string with the name of the emoji (e.g. `"duck"`).
//!
//! The [`Svg`](Svg) struct is a
//! [new type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! for the string that hold the SVG image data.

use crate::TwemojiAsset;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub mod codes;

#[cfg(feature = "names")]
pub mod names;

/// New type for the SVG string.
///
/// The `Svg` type is a
/// [new type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// that provides a clear type definition for representing SVG strings.
/// This type is used in the definition of the [`SvgTwemojiAsset`](SvgTwemojiAsset)
/// struct to ensure that the string is unambiguously a representation of an SVG image.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Svg(pub &'static str);

pub type SvgTwemojiAsset = TwemojiAsset<Svg>;

impl SvgTwemojiAsset {
    /// Finds the Twemoji svg string representing the given emoji.
    ///
    /// Returns `None` if no Twemoji is found for given emoji.
    /// The underlying function is a simply `match` with all the emojis mapped to their reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use twemoji_assets::svg::SvgTwemojiAsset;
    ///
    /// assert_eq!(
    ///     SvgTwemojiAsset::from_emoji("🦆"),
    ///     Some(&twemoji_assets::svg::codes::U_1F986)
    /// );
    ///
    /// assert!(SvgTwemojiAsset::from_emoji("not an emoji").is_none());
    /// ```
    ///
    /// # Binary Size
    ///
    /// Utilizing this function leads to a substantial increase in the size of the compiled binary.
    /// This is because the compiler must import the entire table utilized in the `match` of the
    /// function.
    /// As all arms refer to different SVG string slices, all of the graphics will be imported, causing
    /// the substantially increased binary file size.
    ///
    /// **Different file sizes on Windows (x86_64-pc-windows-msvc):**
    ///
    /// [small example]: https://github.com/cptpiepmatz/twemoji-assets/blob/main/examples/svg_small_binary.rs
    /// [large example]: https://github.com/cptpiepmatz/twemoji-assets/blob/main/examples/svg_large_binary.rs
    ///
    /// |         | [small example][small example] | [large example][large example] | increase |
    /// |---------|--------------------------------|--------------------------------|----------|
    /// | Debug   | 162 KB                         | 8766 KB                        | 5541%    |
    /// | Release | 158 KB                         | 8681 KB                        | 5494%    |
    ///
    /// Therefore this function should only be used if the icon is chosen on runtime and every emoji is
    /// a possible input.
    #[inline]
    pub fn from_emoji(emoji: &str) -> Option<&'static SvgTwemojiAsset> {
        codes::from_emoji(emoji)
    }

    /// Find the Twemoji svg string representing the given shortcode.
    ///
    /// Returns `None` if no Twemoji is found for given shortcode.
    /// The underlying function is a simply `match` with all the shortcodes mapped to their reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use twemoji_assets::svg::SvgTwemojiAsset;
    ///
    /// assert_eq!(
    ///     SvgTwemojiAsset::from_name("duck"),
    ///     Some(&twemoji_assets::svg::codes::U_1F986)
    /// );
    ///
    /// assert!(SvgTwemojiAsset::from_name("not an emoji").is_none());
    /// ```
    ///
    /// # Binary Size
    ///
    /// Using this function causes the built binary to significantly increase in size.
    /// Check [`Self::from_emoji`] for further explanation.
    ///
    #[inline]
    #[cfg(feature = "names")]
    pub fn from_name(shortcode: &str) -> Option<&'static SvgTwemojiAsset> {
        names::from_name(shortcode)
    }
}

impl Deref for SvgTwemojiAsset {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

impl Debug for SvgTwemojiAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SvgTwemojiAsset")
            .field("data", &"<svg>...</svg>")
            .field("emoji", &self.emoji)
            .field("label", &self.label)
            .finish()
    }
}

#[cfg(feature = "png")]
impl From<&super::png::PngTwemojiAsset> for &SvgTwemojiAsset {
    fn from(value: &super::png::PngTwemojiAsset) -> Self {
        match SvgTwemojiAsset::from_emoji(value.emoji) {
            Some(asset) => asset,
            None => unreachable!("PNG and SVG have the same emoji set")
        }
    }
}

macro_rules! svg_code {
    ($ident:ident, $emoji:literal, $label:literal, $file_name:literal) => {
        #[doc = concat!(
            r#"<p style="display: flex; flex-direction: row; align-items: center; gap: 0.5em">"#,
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/svg/"#,
            $file_name,
            r#"" loading="lazy" style="height: 1.2em"></img><span>"#,
            $label,
            r#"</span></p>"#
        )]
        ///
        ///
        /// A constant reference to the string holding the SVG information for the
        #[doc = $emoji]
        /// emoji.
        ///
        #[doc = concat!(
            "The file originates from the [Twemoji collection by Twitter](",
            "https://github.com/twitter/twemoji/blob/master/assets/svg/",
            $file_name,
            ")."
        )]
        ///
        #[doc = concat!(
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/svg/"#,
            $file_name,
            r#"" style="max-height: 20em"></img>"#
        )]
        pub const $ident: SvgTwemojiAsset = SvgTwemojiAsset {
            data: Svg(include_str!(concat!(
                "../../assets/svg/",
                $file_name
            ))),
            emoji: $emoji,
            label: Some($label)
        };
    }
}

macro_rules! svg_name {
    ($alias:ident, $emoji:literal, $label:literal, $redirect:ident, $file_name:literal) => {
        #[doc = concat!(
            r#"<p style="display: flex; flex-direction: row; align-items: center; gap: 0.5em">"#,
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/svg/"#,
            $file_name,
            r#"" loading="lazy" style="height: 1.2em"></img><span>"#,
            $label,
            r#"</span></p>"#
        )]
        ///
        ///
        /// A constant reference to the string holding the SVG information for the
        #[doc = $emoji]
        /// emoji.
        ///
        #[doc = concat!(
            "The file originates from the [Twemoji collection by Twitter](",
            "https://github.com/twitter/twemoji/blob/master/assets/svg/",
            $file_name,
            ")."
        )]
        ///
        #[doc = concat!(
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/svg/"#,
            $file_name,
            r#"" style="max-height: 20em"></img>"#
        )]
        pub const $alias: SvgTwemojiAsset = $redirect;
    }
}

macro_rules! svg_match_emoji {
    [$(($pat:pat, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static SvgTwemojiAsset> {
            let mut emoji = emoji.chars();
            let c0: Option<char> = emoji.next();
            let c1: Option<char> = emoji.next();
            let c2: Option<char> = emoji.next();
            let c3: Option<char> = emoji.next();
            let c4: Option<char> = emoji.next();
            let c5: Option<char> = emoji.next();
            let c6: Option<char> = emoji.next();
            let c7: Option<char> = emoji.next();
            let c8: Option<char> = emoji.next();
            let c9: Option<char> = emoji.next();
            match (c0, c1, c2, c3, c4, c5, c6, c7, c8, c9) {
                $($pat => Some(&$ident),)+
                _ => None
            }
        }
    }
}

macro_rules! svg_match_name {
    [$(($pat:pat, $ident:ident),)+] => {
        pub(super) fn from_name(emoji: &str) -> Option<&'static SvgTwemojiAsset> {
            let char_count = emoji.chars().count();
            match (char_count, emoji) {
                $($pat => Some(&$ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use svg_code;
pub(crate) use svg_match_emoji;
pub(crate) use svg_match_name;
pub(crate) use svg_name;
