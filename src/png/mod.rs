pub mod codes;

#[cfg(feature = "shortcodes")]
pub mod shortcodes;

/// Finds the Twemoji png string representing the given emoji.
///
/// Returns `None` if no Twemoji is found for given emoji.
/// The underlying function is a simply `match` with all the emojis mapped to their reference.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     twemoji_assets::png::from_emoji("🦆"),
///     Some(twemoji_assets::png::codes::U_1F986)
/// );
///
/// assert!(twemoji_assets::png::from_emoji("abc").is_none());
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
pub fn from_emoji(emoji: &str) -> Option<&'static [u8]> {
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
/// assert_eq!(
///     twemoji_assets::png::from_shortcode("duck"),
///     Some(twemoji_assets::png::codes::U_1F986)
/// );
///
/// assert!(twemoji_assets::png::from_shortcode("not an emoji").is_none());
/// ```
///
/// # Binary Size
///
/// Using this function causes the built binary to significantly increase in size.
/// Check [`from_emoji`] for further explanation.
///
#[inline]
#[cfg(feature = "shortcodes")]
pub fn from_shortcode(shortcode: &str) -> Option<&'static [u8]> {
    shortcodes::from_shortcode(shortcode)
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
        pub const $ident: &[u8] = include_bytes!(concat!(
            "../../assets/72x72/",
            $file_name
        ));
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
        pub const $alias: &[u8] = $redirect;
    }
}

macro_rules! png_match_emoji {
    [$(($emoji:literal, $ident:ident),)+] => {
        pub(super) fn from_emoji(emoji: &str) -> Option<&'static [u8]> {
            match emoji {
                $($emoji => Some($ident),)+
                _ => None
            }
        }
    }
}

macro_rules! png_match_shortcode {
    [$(($shortcode:literal, $ident:ident),)+] => {
        pub(super) fn from_shortcode(emoji: &str) -> Option<&'static [u8]> {
            match emoji {
                $($shortcode => Some($ident),)+
                _ => None
            }
        }
    }
}

pub(crate) use png_code;
pub(crate) use png_name;
pub(crate) use png_match_emoji;
pub(crate) use png_match_shortcode;
