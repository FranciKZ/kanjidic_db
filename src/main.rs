use std::fs::File;
use std::time::Instant;
use std::{fs};
use std::fmt::Write;
use std::io::Write as OtherWrite;

use dictionary::Dictionary;
mod dictionary;

fn get_result(content: String) -> std::string::String {
  let doc: Dictionary = serde_xml_rs::from_str(&*content).unwrap();
  let mut res = String::new(); 

  for c in doc.character {
    writeln!(&mut res, "INSERT INTO kanji_db.kanji (literal) VALUES ('{}');", c.literal).unwrap();

    for cp in c.codepoint.values {
      writeln!(&mut res, "INSERT INTO kanji_db.codepoint (code_type, code, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", cp.code_type, cp.code, c.literal).unwrap();
      // codepoints.insert(c.literal.clone(), cp);
    }

    for rad in c.radical.rad_value {
      writeln!(&mut res, "INSERT INTO kanji_db.radical (radical_type, radical_id, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", rad.rad_type, rad.id, c.literal).unwrap();
    }

    match c.reading_meaning {
      Some(x) => {
        for group in x.rm_group {
          match group.meaning {
            Some(g) => {
              for meaning in g {
                match meaning.m_lang {
                  Some(lang) => {
                    writeln!(&mut res, "INSERT INTO kanji_db.meaning (meaning, lang, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", meaning.meaning.replace("'", "''"), lang, c.literal).unwrap();
                  }
                  None => {
                    writeln!(&mut res, "INSERT INTO kanji_db.meaning (meaning, lang, kanji_id) VALUES('{}', NULL, (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", meaning.meaning.replace("'", "''"), c.literal).unwrap();
                  }
                }
              }
            }
            None => ()
          }
          match group.reading {
            Some(g) => {
              for reading in g {
                writeln!(&mut res, "INSERT INTO kanji_db.reading (reading, reading_type, kanji_id) VALUES('{}', '{}', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", reading.reading.replace("'", "''"), reading.r_type, c.literal).unwrap();
              }
            }
            None => ()
          }
        }
        match x.nanori {
          Some(nanoris) => {
            for nanori in nanoris {
              writeln!(&mut res, "INSERT INTO kanji_db.reading (reading, reading_type, kanji_id) VALUES('{}', 'nanori', (SELECT id FROM kanji_db.kanji WHERE literal = '{}'));", nanori, c.literal).unwrap();
            }
          }
          None => ()
        }
      }
      None => ()
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

fn main() {
  let file_name: &str = "./kanjidic2.xml";
  let content: String = fs::read_to_string(file_name).expect("Unable to read file");
  
  let now = Instant::now();

  write_result(get_result(content));
  
  let elapsed_time = now.elapsed();
  println!("Ran in {} seconds", elapsed_time.as_secs());
}
