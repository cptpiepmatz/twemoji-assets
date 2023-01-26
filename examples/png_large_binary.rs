fn main() {
    let duck_emoji_png = twemoji_assets::png::from_emoji("🦆").unwrap();
    for byte in duck_emoji_png {
        println!("{byte}");
    }
}
