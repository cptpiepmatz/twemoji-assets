pub mod codes;

#[cfg(feature = "shortcodes")]
pub mod shortcodes;

pub type Svg = &'static str;
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
    /// assert_eq!(
    ///     twemoji_assets::svg::from_emoji("🦆"),
    ///     Some(twemoji_assets::svg::codes::U_1F986)
    /// );
    ///
    /// assert!(twemoji_assets::svg::from_emoji("abc").is_none());
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
    /// assert_eq!(
    ///     twemoji_assets::svg::from_shortcode("duck"),
    ///     Some(twemoji_assets::svg::codes::U_1F986)
    /// );
    ///
    /// assert!(twemoji_assets::svg::from_shortcode("not an emoji").is_none());
    /// ```
    ///
    /// # Binary Size
    ///
    /// Using this function causes the built binary to significantly increase in size.
    /// Check [`from_emoji`] for further explanation.
    ///
    #[inline]
    #[cfg(feature = "shortcodes")]
    pub fn from_shortcode(shortcode: &str) -> Option<&'static SvgTwemojiAsset> {
        shortcodes::from_shortcode(shortcode)
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
            data: include_str!(concat!(
                "../../assets/svg/",
                $file_name
            )),
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
    [$(($emoji:literal, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static SvgTwemojiAsset> {
            match emoji {
                $($emoji => Some(&$ident),)+
                _ => None
            }
        }
    }
}

macro_rules! svg_match_shortcode {
    [$(($shortcode:literal, $ident:ident),)+] => {
        pub(super) fn from_shortcode(emoji: &str) -> Option<&'static SvgTwemojiAsset> {
            match emoji {
                $($shortcode => Some(&$ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use svg_code;
pub(crate) use svg_name;
pub(crate) use svg_match_emoji;
pub(crate) use svg_match_shortcode;
use crate::TwemojiAsset;
