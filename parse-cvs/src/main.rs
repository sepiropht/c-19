use bson::{bson, doc};
use bson::TimeStamp;
use mongodb::{options::ClientOptions, Client};
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::process;
extern crate chrono;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Deserialize)]
struct Record {
    nom: String,
    prenom: String,
    date_naissance: String,
    code_lieu_naissance: Option<u32>,
    sexe: u32,
    pays_naissance: Option<String>,
    date_deces: TimeStamp,
    code_lieu_deces: Option<u32>,
    numero_acte_deces: Option<String>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut client_options = ClientOptions::parse("mongodb+srv://admin:qUDJuMMCM7opjIQI@cluster0-pap5m.mongodb.net/test?retryWrites=true&w=majority")?;
    let client = Client::with_options(client_options)?;
    let db = client.database("death_france");

    // List the names of the collections in that database.
    let collection = db.collection("person");

    for collection_name in db.list_collection_names(None)? {
        println!("{}", collection_name);
    }
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        dbg!("yeah");
        let record: Record = result?;
        let doc = doc! { "nom": record.nom, "prenom": record.prenom, "sexe": record.sexe, "date_deces": record.date_deces};

        collection.insert_one(doc, None);

    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
