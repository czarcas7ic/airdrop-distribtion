use std::error::Error;
use std::io;
use std::env;
use std::string::String;
use std::str;

use csv;
use std::mem;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Stakers {
    address: String,
    staked: f64,
}

/// Reads data from a file into a reader and deserializes each record
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str, airdrop_amount: f64) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut rdr = csv::Reader::from_path(path)?;

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    let mut total_staked : f64 = 0.0;
    for result in rdr.deserialize() {
        let record: Stakers = result?;
        total_staked += record.staked;
    }

    let total_stakers = mem::size_of::<Stakers>();
    let multiplier : f64 = (total_stakers as f64) / (total_staked as f64);

    let mut rdr = csv::Reader::from_path(path)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for result in rdr.deserialize() {
        let mut record: Stakers = result?;
        record.staked = (record.staked as f64) * multiplier * airdrop_amount;
        wtr.serialize(&record)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Two args required to run the program");
        std::process::exit(1)
    }

    // First arg is the path to the csv file
    let csv_path = &args[1];
    // Second arg is the total amount of tokens the airdrop will consist of
    let airdrop_amount = args[2].parse::<f64>();


    // If an error occurs print error
    if let Err(e) = read_from_file(csv_path, airdrop_amount.unwrap()) {
        eprintln!("{}", e);
    }
}