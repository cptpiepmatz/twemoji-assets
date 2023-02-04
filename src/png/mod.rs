use crate::TwemojiAsset;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub mod codes;

#[cfg(feature = "names")]
pub mod names;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Png(pub &'static [u8]);
pub type PngTwemojiAsset = TwemojiAsset<Png>;

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
        /// A constant reference to the string holding the png information for the
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
        /// A constant reference to the string holding the png information for the
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
    [$(($emoji:literal, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static PngTwemojiAsset> {
            match emoji {
                $($emoji => Some(&$ident),)+
                _ => None
            }
        }
    }
}

macro_rules! png_match_name {
    [$(($name:literal, $ident:ident),)+] => {
        pub(super) fn from_name(emoji: &str) -> Option<&'static PngTwemojiAsset> {
            match emoji {
                $($name => Some(&$ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use png_code;
pub(crate) use png_match_emoji;
pub(crate) use png_match_name;
pub(crate) use png_name;
