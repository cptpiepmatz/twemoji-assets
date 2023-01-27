fn main() {
    let duck_emoji_png = twemoji_assets::png::PngTwemojiAsset::from_emoji("🦆").unwrap();
    println!("{duck_emoji_png:?}");
}
