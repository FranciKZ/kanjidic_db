use std::{fs};
use std::fmt::Write;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Dictionary {
  character: Vec<Character>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Character {
  literal: String,
  codepoint: Codepoint,
  radical: Radical,
  misc: MiscInfo,
  reading_meaning: Option<ReadingMeaning>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Codepoint {
  #[serde(rename = "cp_value")]
  values: Vec<CpValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CpValue {
  #[serde(rename = "cp_type")]
  code_type: String,
  #[serde(rename = "$value")]
  code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Radical {
  rad_value: Vec<RadValue>
}

#[derive(Debug, Serialize, Deserialize)]
struct RadValue {
  rad_type: String,
  #[serde(rename = "$value")]
  id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MiscInfo {
  grade: Option<i64>,
  stroke_count: Vec<i64>,
  variant: Option<Vec<Variant>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Variant {
  var_type: String,
  #[serde(rename = "$value")]
  code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadingMeaning {
  #[serde(rename = "rmgroup")]
  rm_group: Vec<RmGroup>,
  nanori: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RmGroup {
  reading: Option<Vec<Reading>>,
  meaning: Option<Vec<Meaning>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reading {
  r_type: String,
  #[serde(rename = "$value")]
  reading: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
  m_lang: Option<String>,
  #[serde(rename = "$value")]
  meaning: String,
}

fn main() {
  let file_name: &str = "./kanjidic2.xml";
  let content: String = fs::read_to_string(file_name).expect("Unable to read file");
  let doc: Dictionary = serde_xml_rs::from_str(&*content).unwrap();

  let mut res = String::new(); 
  write!(&mut res, "{}", "DROP TABLE IF EXISTS kanji;\n").unwrap();
  write!(&mut res, "{}", "CREATE TABLE kanji (").unwrap();

  // for (i, ch) in x.chars().enumerate() {
  //     write!(&mut res, "{} {}\n", i, ch).unwrap();
  // }
  
  

}
