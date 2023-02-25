# twemoji-assets

A sophisticated crate that provides the assets from 
[Twemoji](https://github.com/twitter/twemoji).

## Usage

To use this crate, simply include it in your Cargo.toml file and enable the 
desired [features](#feature-flags):

```toml
[dependencies]
twemoji-assets = { version = "1.0", features = ["svg", "png", "names"] }
```

This crate provides a convenient way to access the emoji assets from Twemoji in 
your Rust programs. 
You can use the provided `from_emoji` and `from_name` methods to select the 
correct asset at runtime, or use macros to select the right emoji assets at 
compile time.

Here's a simple example that shows how to use this crate to retrieve the SVG and 
PNG assets for the 🦆 (duck) emoji:

```rust
use twemoji_assets::svg::SvgTwemojiAsset;

fn main() {
    let svg_asset: &SvgTwemojiAsset = SvgTwemojiAsset::from_emoji("🦆").unwrap();
    let svg_data: &str = &svg_asset;
    println!("SVG data for 🦆: {}", svg_data);

    #[cfg(feature = "png")]
    {
        use twemoji_assets::png::PngTwemojiAsset;

        let png_asset: &PngTwemojiAsset = PngTwemojiAsset::from_emoji("🦆").unwrap();
        let png_data: &[u8] = &png_asset;
        println!("PNG data for 🦆: {:?}", png_data);
    }
}
```

**Note**: To use the `png` module, you need to enable the `png` feature flag in 
your `Cargo.toml` file.

This crate does not provide any direct methods for utilizing the Twemoji assets.
The following additional crates may be necessary to take full advantage of these 
emoji graphics:

- [image](https://crates.io/crates/image) - A crate that allows for image 
  manipulation of PNGs, which could be used in combination with the `png` 
  feature of twemoji-assets.

- [resvg](https://crates.io/crates/resvg) - A crate that provides SVG rendering, 
  which could be used in combination with the `svg` feature of twemoji-assets.

- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) - A 
  crate that provides a way to split a word into its Unicode graphemes, making 
  it useful to determine when to render an emoji.

- [emoji](https://crates.io/crates/emoji) - A crate that provides a way to use 
  emojis in Rust, including Twemoji assets. 
  It allows for emoji lookup by name or codepoint, and includes a wide variety 
  of emoji sources beyond just Twemoji. 
  It can be used in conjunction with twemoji-assets to convert emojis to 
  Twemoji assets.


## Feature Flags

The features that you include in your `Cargo.toml` will affect which assets are 
available to your program. 
The following features are available:

- `svg` (enabled by default): includes the `svg` module, which provides 
  SVG-format emoji assets.

- `png`: includes the `png` module, which provides PNG-format emoji assets.

- `names` (enabled by default): includes the `names` module, which provides 
  mappings between human-readable names for emoji and their Unicode 
  representations.

As the documentation for this crate includes all features, it is recommended that you generate the docs for your project 
yourself. 
To generate the documentation, run the following command:
```sh
cargo doc
```
This will generate documentation for only the features that you have enabled, 
reducing clutter in the generated documentation.

## Version Scheme

This crate follows the semantic versioning scheme as required by the
[Rust documentation](https://doc.rust-lang.org/cargo/reference/semver.html).
The version number is represented as `x.y.z+a.b.c`, where `x.y.z` is the version
of the crate and `a.b.c` is the version of the integrated Twemoji assets.
The `+` symbol is used to separate the two version numbers.
The version of the crate may increase without a corresponding increase in the
version of the integrated Twemoji assets, however, whenever the Twemoji assets
are updated and new assets are added, the crate version will at least increase
in the minor value (`y`).

## Additional Resources

- [Twemoji Homepage](https://twemoji.twitter.com/) - The official homepage for
  Twemoji, which includes detailed information on the project as well as
  instructions on how to include Twemoji assets in your project.

- [Emojibase Homepage](https://emojibase.github.io/) - A comprehensive emoji
  database that includes information on emoji character codes, names, and
  descriptions.
  This crate includes a subset of Emojibase as the `names` module, but the full
  database may be useful for advanced emoji manipulation.

## Licensing

The codebase and names provided by Emojibase for this crate are licensed under 
the MIT License and the included graphics are licensed by Twitter (Copyright 
2020 Twitter, Inc and other contributors) under the
[Creative Commons Attribution 4.0 International (CC-BY 4.0) license](https://creativecommons.org/licenses/by/4.0/).
Proper attribution must be given to Twitter and other contributors if these 
graphics are used or modified.

## Comparison to twemoji-rs

Another crate with similar goals to this crate is
[twemoji-rs](https://crates.io/crates/emoji).
Like twemoji-assets, it provides assets from Twemoji and makes them easily
available in Rust.
However, instead of directly including the assets, twemoji-rs finds the paths
for the particular graphic on the machine where the code compiles.

Pros of twemoji-rs include:

- Assets may be lazily loaded.
- Its API is simpler than twemoji-assets.

Cons of twemoji-rs include:

- Deployment is not possible on a device where the application is not built, as
  the paths are absolute from the directory the app was built in.
- Assets are not part of the built binary, making deployment more challenging.
- emoji-rs only supports PNG.
- emoji-rs has no overview of all available Twemojis.

Overall, the choice of whether to use twemoji-assets or twemoji-rs will depend
on your specific use case and needs.
