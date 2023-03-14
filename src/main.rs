use std::fs::File;
use std::{fs};
use std::fmt::Write;
use std::io::{Write as OtherWrite};
use flate2::read::GzDecoder;
use std::io::Read;

use dictionary::Dictionary;
mod dictionary;

fn get_result(content: String) -> std::string::String {
  let doc: Dictionary = serde_xml_rs::from_str(&*content).unwrap();
  let mut res = String::new(); 

  for c in doc.character {
    let mut insert_statements = String::new();
    let mut has_meanings = false;
    let mut has_readings = false;
    if let Some(rm) = c.reading_meaning {
      writeln!(&mut insert_statements, "INSERT INTO kanji_db.kanji (literal) VALUES ('{}');", c.literal).unwrap();

      for cp in c.codepoint.values {
        writeln!(&mut insert_statements, "INSERT INTO kanji_db.codepoint (code_type, code_value, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", cp.code_type, cp.code, c.literal).unwrap();
      }
  
      for rad in c.radical.rad_value {
        writeln!(&mut insert_statements, "INSERT INTO kanji_db.radical (radical_type, radical_id, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", rad.rad_type, rad.id, c.literal).unwrap();
      }

      for group in rm.rm_group {
        match group.meaning {
          Some(g) => {
            for meaning in g {
              match meaning.m_lang {
                Some(lang) => {
                  has_meanings = true;
                  writeln!(&mut insert_statements, "INSERT INTO kanji_db.meaning (meaning, lang, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", meaning.meaning.replace("'", "''"), lang, c.literal).unwrap();
                }
                None => {
                  writeln!(&mut insert_statements, "INSERT INTO kanji_db.meaning (meaning, lang, kanji_id) VALUES('{}', NULL, (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", meaning.meaning.replace("'", "''"), c.literal).unwrap();
                }
              }
            }
          }
          None => ()
        }
        match group.reading {
          Some(g) => {
            for reading in g {
              has_readings = true;
              writeln!(&mut insert_statements, "INSERT INTO kanji_db.reading (reading, reading_type, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", reading.reading.replace("'", "''"), reading.r_type, c.literal).unwrap();
            }
          }
          None => ()
        }
        
      }

      match rm.nanori {
        Some(nanoris) => {
          for nanori in nanoris {
            writeln!(&mut insert_statements, "INSERT INTO kanji_db.reading (reading, reading_type, kanji_id) VALUES('{}', 'nanori', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", nanori, c.literal).unwrap();
          }
        }
        None => ()
      }

      if let Some(grade) = c.misc.grade {
        if has_meanings && has_readings && grade < 9{
          write!(&mut res, "{}", insert_statements).unwrap();
        }
      }
    }
  }
    
  return res;
}

fn write_result(result_sql: String) {
  let mut other_content = fs::read_to_string("./template.sql").expect("Unable to read file");
  write!(&mut other_content, "\n{}", result_sql).unwrap();
  let mut output = File::create("./create_script.sql").unwrap();
  write!(output, "{}", other_content).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
  let resp = reqwest::blocking::get("http://www.edrdg.org/kanjidic/kanjidic2.xml.gz")?;
  let mut d = GzDecoder::new(resp);
  let mut content = String::new();
  d.read_to_string(&mut content).unwrap();

  write_result(get_result(content));
  Ok(())
}
