extern crate chrono;
use bson::doc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use mongodb::{options::ClientOptions, Client};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    nom: String,
    prenom: String,
    date_naissance: String,
    code_lieu_naissance: Option<u32>,
    sexe: u32,
    pays_naissance: Option<String>,
    date_deces: String,
    code_lieu_deces: Option<u32>,
    numero_acte_deces: Option<String>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let client_options = ClientOptions::parse(&env::var("MONGO_URL")?)?;
    let client = Client::with_options(client_options)?;
    let db = client.database("death_france");

    let collection = db.collection("person");

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        let date = NaiveDate::parse_from_str(&record.date_deces, "%Y-%m-%d")?;
        let ms = NaiveTime::from_hms_milli(12, 34, 56, 789);
        let dt = NaiveDateTime::new(date, ms);
        let doc = doc! { "nom": record.nom, "prenom": record.prenom, "sexe": record.sexe, "datetime": DateTime::from_utc(dt, Utc)};
        collection.insert_one(doc, None)?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
