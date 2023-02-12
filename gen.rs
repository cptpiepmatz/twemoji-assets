//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{fs};
use std::path::Path;

const EMOJIBASE_DATA: &str = include_str!(concat!(
    env!("RUST_SCRIPT_BASE_PATH"),
    "/emojibase.data.json"
));
const EMOJIBASE_SHORTCODES: &str = include_str!(concat!(
    env!("RUST_SCRIPT_BASE_PATH"),
    "/emojibase.shortcodes.json"
));

const INDENT: &str = "    ";

#[derive(Debug, Deserialize)]
struct EmojibaseData {
    label: String,
    hexcode: String,
}

type Shortcodes = Vec<String>;

fn main() -> Result<(), Box<dyn Error>> {
    let emojibase_data: Vec<EmojibaseData> = serde_json::from_str(EMOJIBASE_DATA)?;
    let emojibase_shortcodes: HashMap<String, Shortcodes> = {
        let values: HashMap<String, Value> = serde_json::from_str(EMOJIBASE_SHORTCODES)?;
        let mut shortcodes: HashMap<String, Shortcodes> = HashMap::new();
        for (k, v) in values {
            match v {
                Value::String(s) => shortcodes.insert(k, Vec::from([s])),
                Value::Array(v) => shortcodes.insert(
                    k,
                    v.into_iter()
                        .map(|v| {
                            serde_json::from_value::<String>(v)
                                .expect("shortcodes array values are always strings")
                        })
                        .collect(),
                ),
                _ => panic!("shortcodes value neither string nor array"),
            };
        }
        shortcodes
    };

    let svg_path = Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/assets/svg"));
    let svg_files = fs::read_dir(svg_path)?;
    let svg_files: Vec<String> = {
        let mut files = Vec::new();
        for file in svg_files {
            let file = file?;
            files.push(file.file_name().to_string_lossy().into())
        }
        files
    };

    let mut svg_codes_mod = include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_codes_mod.template.rs"
    )).to_owned();

    let mut svg_match_emoji = String::new();
    let mut svg_shortcodes_mod = include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_names_mod.template.rs"
    )).to_owned();
    let mut svg_match_shortcode = String::new();

    let mut svg_match_emoji_macro = String::new();
    let mut svg_match_emoji_from_name_macro = String::new();

    for file in svg_files {
        let emojibase_name = file.split(".svg").next().unwrap().to_uppercase();
        let emoji: String = emojibase_name
            .split('-')
            .map(|d| u32::from_str_radix(d, 16).unwrap())
            .map(|n| char::from_u32(n).unwrap())
            .collect();
        let label = emojibase_data
            .iter()
            .find(|data| data.hexcode.as_str() == &emojibase_name)
            .map(|data| data.label.as_str())
            .unwrap_or("")
            .to_string();
        let mut emoji_split = emoji.chars();
        let emoji_matcher = format!(
            "({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next(),
            emoji_split.next()
        );
        let ident = format!("U_{}", emojibase_name.replace('-', "_"));
        svg_codes_mod += &format!("svg_code!({ident}, \"{emoji}\", {label:?}, {file:?});\n",);
        svg_match_emoji += &format!("{INDENT}({emoji_matcher}, {ident}),\n");
        svg_match_emoji_macro += &format!("{INDENT}(\"{emoji}\") => {{ &twemoji_assets::svg::codes::{ident} }};\n");

        if let Some(shortcodes) = emojibase_shortcodes.get(&emojibase_name) {
            for shortcode in shortcodes {
                let name_ident = sanitize_ident(shortcode);
                svg_shortcodes_mod += &format!(
                    "svg_name!({name_ident}, \"{emoji}\", {label:?}, {ident}, {file:?});\n"
                );
                let char_count = shortcode.chars().count();
                let name_matcher = format!("({char_count}, {shortcode:?})");
                svg_match_shortcode += &format!("{INDENT}({name_matcher}, {name_ident}),\n");
                svg_match_emoji_from_name_macro += &format!("{INDENT}({shortcode:?}) => {{ &twemoji_assets::svg::codes::{ident} }};\n");
            }
        }
    }

    svg_codes_mod += &format!(include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_match_emoji.template.rs"
    )), svg_match_emoji);
    svg_shortcodes_mod += &format!(include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_match_name.template.rs"
    )), svg_match_shortcode);

    svg_codes_mod += &format!(include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_twemoji_asset.template.rs"
    )), svg_match_emoji_macro);
    svg_shortcodes_mod += &format!(include_str!(concat!(
        env!("RUST_SCRIPT_BASE_PATH"),
        "/templates/svg_twemoji_asset_from_name.template.rs"
    )), svg_match_emoji_from_name_macro);

    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/svg/codes.rs")),
        &svg_codes_mod,
    )?;
    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/svg/names.rs")),
        &svg_shortcodes_mod,
    )?;

    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/png/codes.rs")),
        svg_codes_mod.replace("svg", "png").replace("Svg", "Png"),
    )?;
    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/png/names.rs")),
        svg_shortcodes_mod.replace("svg", "png").replace("Svg", "Png"),
    )?;

    Ok(())
}

fn sanitize_ident(ident: &str) -> String {
    let as_ident = ident.to_uppercase().replace('-', "_").replace('+', "");
    match as_ident.chars().next() {
        Some('A'..='Z') => as_ident,
        _ => format!("X_{as_ident}"),
    }
}
