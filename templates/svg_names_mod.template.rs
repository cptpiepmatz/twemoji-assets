// @generated

//! SVG Twemoji assets with human-readable names.
//!
//! This module provides a subset of the Twemoji assets, which are provided by Twitter in the form
//! of SVG, with human-readable names.
//! The names are sourced from [Emojibase](https://github.com/milesj/emojibase).
//! The full set of Twemoji assets can be found in the [`codes`](super::codes) module.
//!
//! # Naming Convention
//!
//! The names provided by Emojibase may contain characters that are not valid as Rust identifiers.
//! To handle this, the names are sanitized using the `sanitize_ident` function in `gen.rs`.
//! This function performs the following transformations:
//!
//! - Replaces the "-" character with "_".
//! - Removes any "+" characters.
//! - If the first character is not an alphabetic character, a "X_" prefix is added.
//!
//! Here is the implementation of the `sanitize_ident` function:
//!
//! ```
//! fn sanitize_ident(ident: &str) -> String {
//!     let as_ident = ident.to_uppercase().replace('-', "_").replace('+', "");
//!     match as_ident.chars().next() {
//!         Some('A'..='Z') => as_ident,
//!         _ => format!("X_{as_ident}"),
//!     }
//! }
//! ```

use super::{SvgTwemojiAsset, svg_name, svg_match_name};
use super::codes::*;
