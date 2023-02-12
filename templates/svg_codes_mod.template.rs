// @generated

//! SVG Twemoji assets named by Unicode characters.
//!
//! This module contains all Twemoji Assets provided by Twitter in the form of SVG.
//! The assets are named after the unicode characters used to represent the respective emoji.
//!
//! # Naming Convention
//!
//! Each emoji is constructed using one or more unicode characters, represented by hexadecimal codes.
//! To use an emoji, you need to concatenate these codes with underscores between.
//!
//! To ensure that the names are valid identifiers in Rust, the naming convention used in this
//! module is to prefix the concatenated codes with the letter "U".
//! This is necessary because hexadecimal codes may start with a number, which would result in an
//! invalid identifier in Rust.
//!
//! For example, the scientist emoji 🧑‍🔬 is assembled from the characters `0x1F9D1`, `0x200D`, and
//! `0x1F52C`.
//! To use this emoji, you would have to concatenate these codes with underscores, resulting in the
//! name [`U_1F9D1_200D_1F52C`](U_1F9D1_200D_1F52C).

use super::{SvgTwemojiAsset, svg_code, svg_match_emoji, Svg};
