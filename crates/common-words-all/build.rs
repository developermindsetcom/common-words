use anyhow::Result;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const PATTERN: &str = "../../data/ngrams/**/*.csv";
const OUTPUT_PATH: &str = "src/generated";
const OUTPUT_MOD: &str = "mod.rs";

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    ngram: String,
    freq: f64,
    cumshare: Option<f64>,
    en: Option<String>,
}

fn clean_name(name: String) -> String {
    let clean = name
        .split("-")
        .into_iter()
        .map(|chunk| chunk.into())
        .collect::<Vec<String>>()
        .join("_");
    format!("cwa_{clean}")
}

fn get_language(name:String) -> String {
    let array = name.split("_").into_iter().collect::<Vec<&str>>();
    array[2].to_string()
}

fn get_ngram_size(name:String) -> String {
    let array = name.split("_").into_iter().collect::<Vec<&str>>();
    let ngram = array[1].to_string();

    let size = if ngram.starts_with("1") {
        "one"
    } else if ngram.starts_with("2") {
        "two"
    } else if ngram.starts_with("3") {
        "two"
    } else if ngram.starts_with("4") {
        "two"
    } else if ngram.starts_with("5") {
        "two"
    } else {
        "one" // default
    };

    return size.to_string()
}

fn process_csv(path: &PathBuf) -> Result<()> {
    let name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".csv", "");
    let name = clean_name(name);
    let mut output: String = String::new();
    let mut file = File::create(format!("{OUTPUT_PATH}/{name}.rs").as_str())?;

    output.push_str("use crate::types::Record;\n\n");

    let size = std::fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n")
        .count();
    let size = size - 2; // first line header, last empty line

    output.push_str("#[allow(non_upper_case_globals)]\n");
    output.push_str(format!("pub const {name}:[Record;{}] = [\n", size).as_str());

    let raw = std::fs::read_to_string(path).expect("Should have been able to read the file");
    let mut rdr = csv::Reader::from_reader(raw.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        let ngram = record.ngram.replace("\"", "\\n");
        let cumshare = match record.cumshare {
            Some(cumshare) => format!("Some({})", cumshare),
            None => "None".to_string(),
        };
        let freq = format!("{}", record.freq);
        let freq = match freq.contains(".") {
            true => freq,
            false => format!("{freq}.0"),
        };
        let record_line = format!(
            "Record {{ ngram: \"{}\", freq: {}, cumshare: {}, en: Some(\"{}\") }}",
            ngram,
            freq,
            cumshare,
            record.en.unwrap_or("".into())
        );
        output.push_str(format!("\t{},\n", record_line).as_str());
    }

    output.push_str(format!("];\n").as_str());

    file.write_all(output.as_bytes())?;

    Ok(())
}

fn main() -> Result<()> {
    let mut file = File::create(format!("{OUTPUT_PATH}/{OUTPUT_MOD}").as_str())?;
    let mut output: String = String::new();

    for entry in glob(PATTERN).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".csv", "");
                let name = clean_name(name);
                process_csv(&path)?;

                let language = get_language(name.clone());
                let ngram = get_ngram_size(name.clone());

                output.push_str(format!("#[cfg(any(feature = \"{}\", feature = \"{}\"))]\n", language, ngram).as_str());
                output.push_str(format!("pub mod {};\n", name).as_str());
                output.push_str(format!("#[cfg(any(feature = \"{}\", feature = \"{}\"))]\n", language, ngram).as_str());
                output.push_str(format!("pub use {}::{};\n", name, name).as_str());
            }
            Err(_) => {}
        }
    }

    file.write_all(output.as_bytes())?;

    Ok(())
}

// use anyhow::Result;
// fn main() -> Result<()>{
//     Ok(())
// }
