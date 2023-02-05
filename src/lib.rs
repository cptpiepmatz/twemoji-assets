use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "png")]
pub mod png;

// TODO: ref specific impls here

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
