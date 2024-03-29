#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! A sophisticated crate that provides the assets from Twemoji.
//!
//! [Twemoji](https://github.com/jdecked/twemoji) used to be an open-source project by Twitter that
//! provided Recommended for General Interchange (RGI) emoji images in both SVG and PNG formats.
//! Since the acquisition of Twitter by Elon Musk, the project has moved to its original creators
//! ([source](https://github.com/jdecked/twemoji/issues/10)).
//! This crate makes it easy to use these assets in Rust projects.
//!
//! The PNGs are embedded using the [`include_bytes`] macro and the SVGs are
//! embedded using the [`include_str`] macro.
//! Assets can be accessed either by their Unicode representation in the [`svg::codes`]
//! and [`png::codes`] modules or by their human-readable names from
//! [Emojibase](https://github.com/milesj/emojibase) in the [`svg::names`] and
//! [`png::names`] modules.
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
//! # Example
//!
//! Here's a simple example that shows how to use this crate to retrieve the SVG and PNG assets for
//! the 🦆 (duck) emoji:
//!
//! ```
//! use twemoji_assets::svg::SvgTwemojiAsset;
//!
//! let svg_asset: &SvgTwemojiAsset = SvgTwemojiAsset::from_emoji("🦆").unwrap();
//! let svg_data: &str = &svg_asset;
//! println!("SVG data for 🦆: {:?}", svg_data);
//!
//! #[cfg(feature = "png")]
//! {
//!     use twemoji_assets::png::PngTwemojiAsset;
//!
//!     let png_asset: &PngTwemojiAsset = PngTwemojiAsset::from_emoji("🦆").unwrap();
//!     let png_data: &[u8] = &png_asset;
//!     println!("PNG data for 🦆: {:?}", png_data);
//! }
//! ```
//!
//! # Usage
//!
//! This crate does not provide any direct methods for utilizing the Twemoji assets.
//! The following additional crates may be necessary to take full advantage of these emoji graphics:
//! - [`image`](https://crates.io/crates/image) for PNG image manipulation
//! - [`resvg`](https://crates.io/crates/resvg) for SVG rendering
//! - [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation)
//!   for splitting words into Unicode graphemes
//! - [`emoji`](https://crates.io/crates/emoji) for working with emojis, which can later be
//!   converted to Twemoji assets
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
//!
//! # Comparison to twemoji-rs
//!
//! Another crate with similar goals to this crate is
//! [`twemoji-rs`](https://crates.io/crates/twemoji-rs).
//! It also provides the assets from Twemoji and makes them easily available, but instead of
//! directly including them, it finds the paths for the particular graphic.
//!
//! Pros:
//! - Assets may be lazy loaded.
//! - Simpler API than this crate.
//!
//! Cons:
//! - Deployment is not possible on a device the application is not built on, as the paths are
//!   absolute from the directory the app was built in.
//! - Assets are not part of the built binary, making deployment even more challenging.
//! - `twemoji-rs` only supports PNG.
//! - `twemoji-rs` has no overview of all available Twemojis.

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "png")]
pub mod png;

/// Wrapper struct for the Twemoji assets.
///
/// This struct is the generic form of the Twemoji asset wrapper.
/// The specific implementations of this struct, [`SvgTwemojiAsset`](svg::SvgTwemojiAsset) and
/// [`PngTwemojiAsset`](png::PngTwemojiAsset), provide specialized variants for handling SVGs and
/// PNGs respectively.
///
/// The generic `TwemojiAsset<T>` is mostly used for implementing traits and provides common
/// functionality, while both the specific variants implement the [`Deref`](std::ops::Deref)
/// trait to allow direct access to the underlying asset data stored in the `data` field.
/// This makes it convenient to work with the asset data as if it were a simple reference to the
/// raw data.
#[allow(clippy::derive_ord_xor_partial_ord)]
#[derive(Eq, Ord)]
pub struct TwemojiAsset<T> {
    /// The underlying image data of the Twemoji asset.
    pub data: T,
    /// A string slice representing the emoji character.
    pub emoji: &'static str,
    /// An optional string slice representing the label of the emoji provide by Emojibase.
    pub label: Option<&'static str>,
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

/// Inserts a [`SvgTwemojiAsset`](svg::SvgTwemojiAsset) for a given emoji string literal.
///
/// The `svg_twemoji_asset!` macro takes a string literal of an emoji, such as `"🦆"`, and yields
/// the `&SvgTwemojiAsset` for the [🦆 emoji asset](svg::codes::U_1F986).
/// If the emoji asset is unknown for the given literal, the compilation will fail.
///
/// # Example
///
/// Example for 🦆 (duck):
/// ```
/// use twemoji_assets::svg_twemoji_asset;
///
/// let duck = svg_twemoji_asset!("🦆");
/// println!("SVG data for 🦆: {:?}", duck);
/// ```
#[cfg(all(doc, feature = "svg"))]
#[macro_export]
macro_rules! svg_twemoji_asset {
    ($emoji:literal) => {};
}

/// Inserts a [`SvgTwemojiAsset`](svg::SvgTwemojiAsset) from a string literal of an emoji name.
///
/// The `svg_twemoji_asset_from_name!` macro takes a string literal of an emoji name, such as
/// `"duck"`, and yields the `&SvgTwemojiAsset` for the corresponding
/// [🦆 emoji asset](svg::codes::U_1F986).
/// The names are provided by [Emojibase](https://github.com/milesj/emojibase).
/// If the name or the corresponding emoji asset is unknown for the given literal, the compilation will fail.
///
/// # Example
///
/// Example for 🦆 (duck):
/// ```
/// use twemoji_assets::svg_twemoji_asset_from_name;
///
/// let duck = svg_twemoji_asset_from_name!("duck");
/// println!("SVG data for 🦆: {:?}", duck);
/// ```
#[cfg(all(doc, feature = "svg", feature = "names"))]
#[macro_export]
macro_rules! svg_twemoji_asset_from_name {
    ($emoji:literal) => {};
}

/// Inserts a [`PngTwemojiAsset`](png::PngTwemojiAsset) for a given emoji string literal.
///
/// The `png_twemoji_asset!` macro takes a string literal of an emoji, such as `"🦆"`, and yields
/// the `&PngTwemojiAsset` for the [🦆 emoji asset](png::codes::U_1F986).
/// If the emoji asset is unknown for the given literal, the compilation will fail.
///
/// # Example
///
/// Example for 🦆 (duck):
/// ```
/// use twemoji_assets::png_twemoji_asset;
///
/// let duck = png_twemoji_asset!("🦆");
/// println!("PNG data for 🦆: {:?}", duck);
/// ```
#[cfg(all(doc, feature = "png"))]
#[macro_export]
macro_rules! png_twemoji_asset {
    ($emoji:literal) => {};
}

/// Inserts a [`PngTwemojiAsset`](png::PngTwemojiAsset) from a string literal of an emoji name.
///
/// The `png_twemoji_asset_from_name!` macro takes a string literal of an emoji name, such as
/// `"duck"`, and yields the `&PngTwemojiAsset` for the corresponding
/// [🦆 emoji asset](svg::codes::U_1F986).
/// The names are provided by [Emojibase](https://github.com/milesj/emojibase).
/// If the name or the corresponding emoji asset is unknown for the given literal, the compilation will fail.
///
/// # Example
///
/// Example for 🦆 (duck):
/// ```
/// use twemoji_assets::png_twemoji_asset_from_name;
///
/// let duck = png_twemoji_asset_from_name!("duck");
/// println!("PNG data for 🦆: {:?}", duck);
/// ```
#[cfg(all(doc, feature = "png", feature = "names"))]
#[cfg(doc)]
#[macro_export]
macro_rules! png_twemoji_asset_from_name {
    ($emoji:literal) => {};
}
