fn main() {
    let duck_emoji_svg = twemoji_assets::svg::SvgTwemojiAsset::from_emoji("🦆").unwrap();
    println!("{duck_emoji_svg:?}");
}
