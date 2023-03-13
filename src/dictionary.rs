use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
  pub character: Vec<Kanji>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kanji {
  pub literal: String,
  pub codepoint: Codepoint,
  pub radical: Radical,
  pub misc: MiscInfo,
  pub reading_meaning: Option<ReadingMeaning>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Codepoint {
  #[serde(rename = "cp_value")]
  pub values: Vec<CpValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpValue {
  #[serde(rename = "cp_type")]
  pub code_type: String,
  #[serde(rename = "$value")]
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radical {
  pub rad_value: Vec<RadValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadValue {
  pub rad_type: String,
  #[serde(rename = "$value")]
  pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiscInfo {
  pub grade: Option<i32>,
  pub stroke_count: Vec<i32>,
  pub variant: Option<Vec<Variant>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
  pub var_type: String,
  #[serde(rename = "$value")]
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingMeaning {
  #[serde(rename = "rmgroup")]
  pub rm_group: Vec<RmGroup>,
  pub nanori: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RmGroup {
  pub reading: Option<Vec<Reading>>,
  pub meaning: Option<Vec<Meaning>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reading {
  pub r_type: String,
  #[serde(rename = "$value")]
  pub reading: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meaning {
  pub m_lang: Option<String>,
  #[serde(rename = "$value")]
  pub meaning: String,
}
