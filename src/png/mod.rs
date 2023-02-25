//! PNG Twemoji assets.
//!
//! This module provides access to the Twemoji assets in PNG format.
//! All emojis are 72x72 pixels and are sourced from the
//! [Twemoji repository](https://github.com/twitter/twemoji/tree/master/assets/72x72).
//!
//! The module has two main components:
//! - The [`codes`](codes) module, which provides all assets named by their unicode characters.
//! - The [`names`](names) module, which provides a subset of emojis with human-readable names.
//!   This module requires the `names` feature to be enabled.
//!
//! The main type to use when interacting with the module is the
//! [`PngTwemojiAsset`](PngTwemojiAsset) type definition.
//! It provides convenient methods for loading assets from either a string containing the emoji's
//! unicode character (e.g. `"🦆"`) or the string with the name of the emoji (e.g. `"duck"`).
//!
//! The [`Png`](Png) struct is a
//! [new type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! for the bytes that hold the PNG image data.

use crate::TwemojiAsset;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub mod codes;

#[cfg(feature = "names")]
pub mod names;

/// New type for the PNG bytes.
///
/// The `Png` type is a
/// [new type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// that provides a clear type definition for representing PNG bytes.
/// This type is used in the definition of the [`PngTwemojiAsset`](PngTwemojiAsset)
/// struct to ensure that the bytes are unambiguously a representation of an PNG image.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Png(pub &'static [u8]);

/// Specific version of [`TwemojiAsset`](super::TwemojiAsset) that holds the PNG bytes of an emoji
/// asset.
///
/// This type is an alias for `TwemojiAsset` with [`Png`](Png) as the type parameter.
/// The `PngTwemojiAsset` implements [`Deref`](std::ops::Deref) to directly return the underlying
/// PNG bytes as a static [`u8`](u8) slice.
pub type PngTwemojiAsset = TwemojiAsset<Png>;

/// The implementation for selecting [`PngTwemojiAsset`s](PngTwemojiAsset) in runtime.
impl PngTwemojiAsset {
    /// Finds the Twemoji png string representing the given emoji.
    ///
    /// Returns `None` if no Twemoji is found for given emoji.
    /// The underlying function is a simply `match` with all the emojis mapped to their reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use twemoji_assets::png::PngTwemojiAsset;
    ///
    /// assert_eq!(
    ///     PngTwemojiAsset::from_emoji("🦆"),
    ///     Some(&twemoji_assets::png::codes::U_1F986)
    /// );
    ///
    ///     assert!(PngTwemojiAsset::from_emoji("not an emoji").is_none());
    /// ```
    ///
    /// # Binary Size
    ///
    /// Utilizing this function leads to a substantial increase in the size of the compiled binary.
    /// This is because the compiler must import the entire table utilized in the `match` of the
    /// function.
    /// As all arms refer to different png string slices, all of the graphics will be imported, causing
    /// the substantially increased binary file size.
    ///
    /// **Different file sizes on Windows (x86_64-pc-windows-msvc):**
    ///
    /// [small example]: https://github.com/cptpiepmatz/twemoji-assets/blob/main/examples/png_small_binary.rs
    /// [large example]: https://github.com/cptpiepmatz/twemoji-assets/blob/main/examples/png_large_binary.rs
    ///
    /// |         | [small example][small example] | [large example][large example] | increase |
    /// |---------|--------------------------------|--------------------------------|----------|
    /// | Debug   | 162 KB                         | 3670 KB                        | 2265%    |
    /// | Release | 157 KB                         | 3585 KB                        | 2283%    |
    ///
    /// *Note*: Numbers may differ but the increase is still relevant.
    ///
    /// Therefore this function should only be used if the icon is chosen on runtime and every emoji is
    /// a possible input.
    #[inline]
    pub fn from_emoji(emoji: &str) -> Option<&'static PngTwemojiAsset> {
        codes::from_emoji(emoji)
    }

    /// Find the Twemoji png string representing the given shortcode.
    ///
    /// Returns `None` if no Twemoji is found for given shortcode.
    /// The underlying function is a simply `match` with all the shortcodes mapped to their reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use twemoji_assets::png::PngTwemojiAsset;
    ///
    /// assert_eq!(
    ///     PngTwemojiAsset::from_name("duck"),
    ///     Some(&twemoji_assets::png::codes::U_1F986)
    /// );
    ///
    /// assert!(PngTwemojiAsset::from_name("not an emoji").is_none());
    /// ```
    ///
    /// # Binary Size
    ///
    /// Using this function causes the built binary to significantly increase in size.
    /// Check [`Self::from_emoji`] for further explanation.
    ///
    #[inline]
    #[cfg(any(feature = "names", test))]
    pub fn from_name(shortcode: &str) -> Option<&'static PngTwemojiAsset> {
        names::from_name(shortcode)
    }
}

impl Deref for PngTwemojiAsset {
    type Target = &'static [u8];

    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

impl Debug for PngTwemojiAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PngTwemojiAsset")
            .field("data", &"[png bytes...]")
            .field("emoji", &self.emoji)
            .field("label", &self.label)
            .finish()
    }
}

#[cfg(feature = "svg")]
impl From<&super::svg::SvgTwemojiAsset> for &PngTwemojiAsset {
    fn from(value: &super::svg::SvgTwemojiAsset) -> Self {
        match PngTwemojiAsset::from_emoji(value.emoji) {
            Some(asset) => asset,
            None => unreachable!("PNG and SVG have the same emoji set")
        }
    }
}

macro_rules! png_code {
    ($ident:ident, $emoji:literal, $label:literal, $file_name:literal) => {
        #[doc = concat!(
            r#"<p style="display: flex; flex-direction: row; align-items: center; gap: 0.5em">"#,
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/72x72/"#,
            $file_name,
            r#"" loading="lazy" style="height: 1.2em"></img><span>"#,
            $label,
            r#"</span></p>"#
        )]
        ///
        ///
        /// A constant reference to the bytes holding the png information for the
        #[doc = $emoji]
        /// emoji.
        ///
        #[doc = concat!(
            "The file originates from the [Twemoji collection by Twitter](",
            "https://github.com/twitter/twemoji/blob/master/assets/72x72/",
            $file_name,
            ")."
        )]
        ///
        #[doc = concat!(
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/72x72/"#,
            $file_name,
            r#"" style="max-height: 20em"></img>"#
        )]
        pub const $ident: PngTwemojiAsset = PngTwemojiAsset {
            data: Png(include_bytes!(concat!(
                "../../assets/72x72/",
                $file_name
            ))),
            emoji: $emoji,
            label: Some($label)
        };
    }
}

macro_rules! png_name {
    ($alias:ident, $emoji:literal, $label:literal, $redirect:ident, $file_name:literal) => {
        #[doc = concat!(
            r#"<p style="display: flex; flex-direction: row; align-items: center; gap: 0.5em">"#,
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/72x72/"#,
            $file_name,
            r#"" loading="lazy" style="height: 1.2em"></img><span>"#,
            $label,
            r#"</span></p>"#
        )]
        ///
        ///
        /// A constant reference to the bytes holding the png information for the
        #[doc = $emoji]
        /// emoji.
        ///
        #[doc = concat!(
            "The file originates from the [Twemoji collection by Twitter](",
            "https://github.com/twitter/twemoji/blob/master/assets/72x72/",
            $file_name,
            ")."
        )]
        ///
        #[doc = concat!(
            r#"<img src="https://raw.githubusercontent.com/cptpiepmatz/twemoji-assets/main/assets/72x72/"#,
            $file_name,
            r#"" style="max-height: 20em"></img>"#
        )]
        pub const $alias: PngTwemojiAsset = $redirect;
    }
}

macro_rules! png_match_emoji {
    [$(($pat:pat, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static PngTwemojiAsset> {
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

macro_rules! png_match_name {
    [$(($pat:pat, $ident:ident),)+] => {
        pub(super) fn from_name(emoji: &str) -> Option<&'static PngTwemojiAsset> {
            let char_count = emoji.chars().count();
            match (char_count, emoji) {
                $($pat => Some(&$ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use png_code;
pub(crate) use png_match_emoji;
pub(crate) use png_match_name;
pub(crate) use png_name;
