fn main() {
    let duck_emoji_png = twemoji_assets::png::shortcodes::DUCK;
    for byte in duck_emoji_png {
        println!("{byte}");
    }
}
