extern crate base64;
//extern crate byte_sha;
#[macro_use]
extern crate clap;
extern crate md5;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;
extern crate snap;
extern crate toml;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Bytes;
use std::io::prelude::*;
use std::path::Path;

use base64::{encode, decode};
use clap::{Arg, App};
use prettytable::Table;
use snap::{Encoder, Decoder};

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    size: u64,
    samples: Vec<Sample>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    filename: String,
    data: String,
    md5: String,
    sha256: String,
}

fn compute_md5(bytes: &[u8]) -> String {
    println!("Computing md5 sum.");
    format!("{:x}", md5::compute(bytes))
}

// fn compute_sha256() {
//     println!("Computing sha256 sum");
//     unimplemented!()
// }

fn make_safe(file: Bytes<File>) -> (String,String,String) {
    println!("Encoding sample binary.");
    let mut byte_vec: Vec<u8> = vec!();
    for byte in file {
        byte_vec.push(byte.expect("Invalid byte."));
    }
    let byte_arr = byte_vec.as_slice();
    let compressed: Vec<u8> = Encoder::compress_vec(&mut Encoder::new(), byte_arr).expect("Failed to compress binary.");
    (encode(&compressed.as_slice()), compute_md5(&compressed.as_slice()), String::new())
}

fn make_live(b64: &String) -> Vec<u8> {
    println!("Decoding sample binary.");
    Decoder::decompress_vec(&mut Decoder::new(), decode(&b64).expect("Failed to decode binary.").as_slice()).expect("Failed to decompress binary.")
}

fn read_bytes(path: &str) -> Bytes<File> {
    println!("Reading file.");
    let path = Path::new(path);
    OpenOptions::new().read(true).open(path).expect("Failed to read sample bytes.").bytes()
}

fn load_db(db: Database) -> Database {
    println!("Loading database.");
    let path = Path::new("mal_database.toml");
    let mut db_file = match File::open(&path) {
        Err(why) => {
            let temp = File::create("mal_database.toml").expect("Failed to create file.").write(toml::to_string(&db).expect("Failed to write to file.").as_bytes());
            match temp {
                Ok(o) => println!("Read database successfully: {}", o),
                Err(e) => println!("Failed to read database: {} {}", why.description(), e.description()),
            }
            panic!("Failed to read database. Creating.");
        },
        Ok(file) => file,
    };
    let mut db_strings: String = String::new();
    db_file.read_to_string(&mut db_strings).expect("Failed to read file.");
    toml::from_str(&db_strings.as_str()).expect("Failed to parse database.")
}

fn save_db(db: Database) {
    println!("Saving database.");
    let db_strings: String = match toml::to_string(&db) {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to serialize database: {}", e.description());
            String::new()
        }
    };
    match OpenOptions::new().create(true).append(false).write(true).open("mal_database.toml") {
        Ok(ref mut file) => {
            write!(
                file,
                "{}",
                db_strings.as_str()
            ).is_ok();
        },

        Err(e) => {
            println!("Failed to save database: {}", e)
        }
    }

}

fn main() {
    let mut db: Database = Database{samples: vec!(), size: 0};
    db = load_db(db);
    let matches = App::new(crate_name!()).version(crate_version!()).author(crate_authors!()).about(crate_description!()).arg(Arg::with_name("import").short("i").long("import").help("Imports a sample into the database").takes_value(true).required(false)).arg(Arg::with_name("export").short("x").long("export").help("Exports the given sample from the database").takes_value(true).required(false)).arg(Arg::with_name("list").short("l").long("list").help("Lists all samples in database").takes_value(true).required(false)).get_matches();

    match matches.value_of("import") {
        Some(s) => {
            println!("Adding sample to database: {}", s);
            let safe_file: (String, String, String) = make_safe(read_bytes(s));
            let path: &Path = Path::new(s);
            db.samples.push(Sample{filename: String::from(path.file_name().expect("Failed to find filename.").to_str().expect("Failed to parse filename.")), data: safe_file.0, md5: safe_file.1, sha256: safe_file.2});
            db.size = db.size + 1;
            println!("Sample sucessfully added.");
        },
        None => (),
    };

    match matches.value_of("export") {
        Some(s) => {
            println!("Exporting from database: {}", s);
            let mut live_bytes: Vec<u8> = vec!();
            for sample in &db.samples {
                print!(".");
                if &sample.filename == s {
                    live_bytes = make_live(&sample.data);
                } else {
                    println!("Sample not found.");
                }
            }
            match OpenOptions::new().create(true).append(false).write(true).open(s) {
                Ok(ref mut file) => {
                    match file.write(live_bytes.as_slice()){
                        Ok(o) => println!("Successfully wrote live bytes: {}", o),
                        Err(e) => println!("Failed to write bytes: {}", e),
                    }
                    ()
                },
                Err(e) => {
                    println!("Failed to save database: {}", e)
                }
            }
            let temp = File::create(s).expect("Failed to create file. Sample may already exist.").write(live_bytes.as_slice());
            match temp {
                Ok(o) => println!("File created successfully: {}", o),
                Err(e) => println!("Failed to create file: {}", e.description()),
            }
            println!("Successfully exported sample.");
        },
        None => (),
    };

    match matches.value_of("list") {
        Some(s) => {
            let mut table = Table::new();
            table.add_row(row!["Filename", "md5", "sha256"]);
            for sample in &db.samples {
                if sample.filename == s {
                    table.add_row(row![&sample.filename.as_str(), &sample.md5.as_str(), &sample.sha256.as_str()]);
                }
            }
            table.printstd();
        },
        None => {
            let mut table = Table::new();
            table.add_row(row!["Filename", "md5", "sha256"]);
            if matches.is_present("list") {
                for sample in &db.samples {
                    table.add_row(row![&sample.filename.as_str(), &sample.md5.as_str(), &sample.sha256.as_str()]);
                }
                table.printstd();
            }
        }
    }

    save_db(db);
}
