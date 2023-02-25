# Kanjidic_DB

WIP project, please don't use :)

The end goal of this is to be able to easily create a SQL db structured around kanjidic2. There are a couple projects like this, but I wanted to try my hand at it to learn a little bit of Rust. I also will eventually use it for a side project.

For a project that seems to be fully working see: [edict_database](https://github.com/odrevet/edict_database)

My goals with the project are as follows:
* [ ] Parse kanjidic xml file into structs
* [ ] Create SQL script that will create all needed tables, structure TBD honestly
  * Going with a script instead of creating the DB and everything because I don't like migrating from DB to DB. I want a file that I can copy the contents of and then run against any DB I plan on using. I don't want to do like `SHOW CREATE TABLE` and copy that then run against my own db or anything.
* [ ] Maybe offer a way to give a connection string to have the script auto run?
* [ ] Download latest kanjidic file for you and unzip it
* [ ] Provide command line arguments to pick and choose which properties you want to parse
  * Maybe command line arguments for different SQL syntaxes as well
* [ ] Maybe get working with JM Dict and generate a vocabulary table and such as well
* [ ] Learn Rust :)

Honestly a little overkill for something that'll likely be used one time by anyone else who uses it, but 🤷🏼‍♂️🤷🏼‍♂️

## Licensing

This project is in no way affiliated to the kanjidic/edict project and if you use it please be sure to adhere to their licensing rules and restrictions.

For documentation on the kanjidic project please go here:
* https://www.edrdg.org/wiki/index.php/KANJIDIC_Project
* http://www.edrdg.org/
