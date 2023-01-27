use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

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

impl<T> Deref for TwemojiAsset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
