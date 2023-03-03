use std::fs::File;
use std::{fs};
use std::fmt::Write;
use std::io::Write as OtherWrite;

use sea_query::Table;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Dictionary {
  character: Vec<Kanji>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Kanji {
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
  id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MiscInfo {
  grade: Option<i32>,
  stroke_count: Vec<i32>,
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

fn write_result(result_sql: String) {
  let mut other_content = fs::read_to_string("./template.sql").expect("Unable to read file");
  write!(&mut other_content, "\n{}", result_sql).unwrap();
  // let _doc: Dictionary = serde_xml_rs::from_str(&*content).unwrap();
  println!("{}", &other_content);

  let mut output = File::create("./create_script.sql").unwrap();
  write!(output, "{}", other_content).unwrap();
}

fn main() {
  let file_name: &str = "./kanjidic2.xml";
  let content: String = fs::read_to_string(file_name).expect("Unable to read file");
  let _doc: Dictionary = serde_xml_rs::from_str(&*content).unwrap();
  // TODO: Evaluate this structure a little more. It's heavy normalized, but not sure if it actually like...needs to be
  // With ~6000 rows, any db with reasonable hardware and proper indexing should be able to go over a non-normalized version of this rather quickly
  // no matter the column used for look up. Something in my likes the normalization despite the fact that it's gonna require to join on pretty much every query
  // in the span of this comment I feel like I've talked myself out of the normalization...or at least want to see if there's a middle ground
  // TODO: is to see if there's a better way to build up this script?? Unsure, need to look up some rust syntax stuff probably cuz this feels wrong(?)

  // TODO Investigate Disel and Quaint as options to programatically generate SQL instead :)
  let mut res = String::new(); 
  write!(&mut res, "{}", "\n hello").unwrap();
  // TODO: Strat is to, obviously, loop over Dictionary, at each entry try not to do much nested stuff, and then generate insert statements from them.
  // Can I use maps to cut back on nesting? Does rust have maps? 
  
  let table = Table::create()
    .table("kanji")
    .if_not_exists();
  write_result(res);
}
