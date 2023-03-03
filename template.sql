CREATE SCHEMA IF NOT EXISTS kanji_db;

CREATE TABLE IF NOT EXISTS kanji_db.kanji (id INTEGER GENERATED ALWAYS AS IDENTITY, literal TEXT, PRIMARY KEY(id));
CREATE TABLE IF NOT EXISTS kanji_db.codepoint (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, code_type TEXT, code_value TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.radical (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, radical_id TEXT, radical_type TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.additional_info (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, code_type TEXT, code_value TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.radical (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, radical_id TEXT, radical_type TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.meaning (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, meaning TEXT, lang TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.reading (id INTEGER GENERATED ALWAYS AS IDENTITY, kanji_id INTEGER, reading TEXT, reading_type TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(kanji_id) REFERENCES kanji_db.kanji(id));
CREATE TABLE IF NOT EXISTS kanji_db.variant (id INTEGER GENERATED ALWAYS AS IDENTITY, additional_info_id INTEGER, variant_type TEXT, code TEXT, PRIMARY KEY(id), CONSTRAINT fk_kanji FOREIGN KEY(additional_info_id) REFERENCES kanji_db.additional_info(id));
