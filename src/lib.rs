#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! A sophisticated crate that provides the assets from Twemoji.
//!
//! [Twemoji](https://github.com/twitter/twemoji) is an open-source project by Twitter that
//! provides Recommended for General Interchange (RGI) emoji images in both SVG and PNG formats.
//! This crate makes it easy to use these assets in Rust projects.
//!
//! The PNGs are embedded using the [`include_bytes`](std::include_bytes) macro and the SVGs are
//! embedded using the [`include_str`](std::include_str) macro.
//! Assets can be accessed either by their Unicode representation in the [`svg::codes`](svg::codes)
//! and [`png::codes`](png::codes) modules or by their human-readable names from
//! [Emojibase](https://github.com/milesj/emojibase) in the [`svg::names`](svg::names) and
//! [`png::names`](png::names) modules.
//! The `from_emoji` and `from_name` methods in the [`SvgTwemojiAsset`](svg::SvgTwemojiAsset) and
//! [`PngTwemojiAsset`](png::PngTwemojiAsset) modules provide convenient ways to select the correct
//! asset at runtime, and the crate's macros allow the selection of the right emojis at compile
//! time.
//!
//! # Feature Flags
//!
//! For a better documentation experience, it is recommended to use the search feature.
//! Additionally, you can
//! [locally generate documentation](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
//! and select the appropriate features to declutter the search results and improve results found
//! by IntelliSense.
//! This crate uses three feature flags: `svg`, `png`, and `names`.
//! The `svg` and `png` flags include the respective file formats, while the `names` flag includes
//! the modules with human-readable names provided by Emojibase.
//! By default, the `svg` and `names` flags are selected.
//!
//! # Usage
//!
//! This crate does not provide any direct methods for utilizing the Twemoji assets.
//! The following additional crates may be necessary to take full advantage of these emoji graphics:
//! - [`image`](https://crates.io/crates/image) for PNG image manipulation
//! - [`resvg`](https://crates.io/crates/resvg) for SVG rendering
//! - [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation)
//!   for splitting words into Unicode graphemes
//!
//! # Version Scheme
//!
//! This crate follows the semantic versioning scheme as required by the
//! [Rust documentation](https://doc.rust-lang.org/cargo/reference/semver.html).
//! The version number is represented as `x.y.z+a.b.c`, where `x.y.z` is the version of the crate
//! and `a.b.c` is the version of the integrated Twemoji assets.
//! The `+` symbol is used to separate the two version numbers.
//! The version of the crate may increase without a corresponding increase in the version of the
//! integrated Twemoji assets, however, whenever the Twemoji assets are updated and new assets are
//! added, the crate version will at least increase in the minor value (`y`).
//!
//! # Licensing
//!
//! The codebase and names provided by Emojibase for this crate are licensed under the MIT License
//! and the included graphics are licensed by Twitter (Copyright 2020 Twitter, Inc and other
//! contributors) under the
//! [Creative Commons Attribution 4.0 International (CC-BY 4.0) license](https://creativecommons.org/licenses/by/4.0/).
//! Proper attribution must be given to Twitter and other contributors if these graphics are used
//! or modified.

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "png")]
pub mod png;

#[derive(Eq, Ord)]
pub struct TwemojiAsset<T> {
    pub data: T,
    pub emoji: &'static str,
    pub label: Option<&'static str>
}

impl<T> PartialEq for TwemojiAsset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.emoji.eq(other.emoji)
    }
}

impl<T> PartialOrd for TwemojiAsset<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.emoji.partial_cmp(other.emoji)
    }
}

impl<T> Hash for TwemojiAsset<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.emoji.hash(state)
    }
}

// dummy macros for docs

#[cfg(all(doc, feature = "svg"))]
#[macro_export]
macro_rules! svg_twemoji_asset {
    ($emoji:literal) => {}
}

#[cfg(all(doc, feature = "svg", feature = "names"))]
#[macro_export]
macro_rules! svg_twemoji_asset_from_name {
    ($emoji:literal) => {}
}

#[cfg(all(doc, feature = "png"))]
#[macro_export]
macro_rules! png_twemoji_asset {
    ($emoji:literal) => {}
}

#[cfg(all(doc, feature = "png", feature = "names"))]
#[cfg(doc)]
#[macro_export]
macro_rules! png_twemoji_asset_from_name {
    ($emoji:literal) => {}
}
