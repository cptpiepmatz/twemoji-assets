//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

const EMOJIBASE_DATA: &str = include_str!(concat!(
    env!("RUST_SCRIPT_BASE_PATH"),
    "/emojibase.data.json"
));
const EMOJIBASE_SHORTCODES: &str = include_str!(concat!(
    env!("RUST_SCRIPT_BASE_PATH"),
    "/emojibase.shortcodes.json"
));

#[derive(Debug, Deserialize)]
struct EmojibaseData {
    label: String,
    hexcode: String,
    emoji: String,
    text: String,
    #[serde(rename = "type")]
    kind: u32,
    version: f32,
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

    let mut svg_codes_mod =
        String::from("// @generated\nuse super::{svg_code, svg_match_emoji};\n");
    let mut svg_match_emoji = String::new();
    let mut svg_names_mod = String::from(
        "// @generated\nuse super::{svg_name, svg_match_shortcode};\nuse super::codes::*;\n",
    );
    let mut svg_match_shortcode = String::new();
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
        let ident = format!("U_{}", emojibase_name.replace('-', "_"));
        svg_codes_mod += &format!("svg_code!({ident}, \"{emoji}\", \"{label}\", \"{file}\");\n",);
        svg_match_emoji += &format!("    (\"{emoji}\", {ident}),\n");

        if let Some(names) = emojibase_shortcodes.get(&emojibase_name) {
            for name in names {
                let name_ident = sanitize_ident(name);
                svg_names_mod += &format!(
                    "svg_name!({name_ident}, \"{emoji}\", \"{label}\", {ident}, \"{file}\");\n"
                );
                svg_match_shortcode += &format!("    (\"{name}\", {name_ident}),\n");
            }
        }
    }

    svg_codes_mod += &format!("\nsvg_match_emoji! [\n{svg_match_emoji}];");
    svg_names_mod += &format!("\nsvg_match_shortcode! [\n{svg_match_shortcode}];");

    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/svg/codes.rs")),
        svg_codes_mod,
    )?;
    fs::write(
        Path::new(concat!(env!("RUST_SCRIPT_BASE_PATH"), "/src/svg/shortcodes.rs")),
        svg_names_mod,
    )?;

    //
    //     let emojis: Value = serde_json::from_str(EMOJIBASE)?;
    //     let Value::Object(emojis) = emojis else {
    //         return Err("json not an object".into());
    //     };
    //
    //     let mut svg_file = String::from("// @generated\nuse super::{svg_code, svg_name};\n");
    //     for (k, v) in emojis {
    //         let mut code_ident = String::from("U");
    //         let mut emoji = String::new();
    //         let mut file_name = String::new();
    //         for code in k.split('-') {
    //             code_ident += &format!("_{}", code);
    //             let digit: u32 = u32::from_str_radix(code, 16)?;
    //             emoji.push(char::from_u32(digit).ok_or(format!("code {code} not a char"))?);
    //             match file_name.len() {
    //                 0 => file_name += &format!("{digit:x}"),
    //                 _ => file_name += &format!("-{digit:x}"),
    //             }
    //         }
    //
    //         let Ok(true) = Path::new(&format!("{}/assets/svg/{}.svg", env!("RUST_SCRIPT_BASE_PATH"), file_name)).try_exists() else {
    //             continue;
    //         };
    //
    //         svg_file += &format!("svg_code!({code_ident}, \"{emoji}\", \"{file_name}\");\n");
    //         match v {
    //             Value::String(name) => {
    //                 svg_file += &format!(
    //                     "svg_name!({}, \"{emoji}\", {code_ident});\n",
    //                     sanitize_ident(&name)
    //                 )
    //             }
    //             Value::Array(names) => {
    //                 for name in names {
    //                     if let Value::String(name) = name {
    //                         svg_file += &format!(
    //                             "svg_name!({}, \"{emoji}\", {code_ident});\n",
    //                             sanitize_ident(&name)
    //                         )
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     fs::write(
    //         Path::new(concat!(
    //             env!("RUST_SCRIPT_BASE_PATH"),
    //             "/src/svg/generated.rs"
    //         )),
    //         svg_file,
    //     )?;
    //
    Ok(())
}

fn sanitize_ident(ident: &str) -> String {
    let as_ident = ident.to_uppercase().replace('-', "_").replace('+', "");
    match as_ident.chars().next() {
        Some('A'..='Z') => as_ident,
        _ => format!("X_{as_ident}"),
    }
}
