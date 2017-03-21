#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("import")
        .short("i")
        .long("import")
        .value_name("IMPORT_FILE")
        .help("Imports a sample into the database")
        .takes_value(true)
        .required(false))
        .arg(Arg::with_name("export")
        .short("x")
        .long("export")
        .value_name("EXPORT_FILE")
        .help("Exports the given sample from the database")
        .takes_value(true)
        .required(false))
        .arg(Arg::with_name("list")
        .short("l")
        .long("list")
        .help("Lists all samples in database")
        .takes_value(false)
        .required(false))
        .get_matches();

    println!("Importing file: {}", matches.value_of("IMPORT_FILE").unwrap());
    println!("Exporting file: {}", matches.value_of("EXPORT_FILE").unwrap());

    
}
