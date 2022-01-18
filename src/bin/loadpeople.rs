use std::error::Error;
use std::io;
use mysql::prelude::*;


#[path="../db.rs"]
mod db;


/// If an error occurs, the error is returned to `main`.
fn read_from_stdin() -> std::result::Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from `stdin`
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

		let mut  conn = db::get_conn().unwrap();
    // let headers = reader.headers()?;

    // println!("Headers: {:?}", headers);

    // `.records` return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result.unwrap();

        // println!("{:?}", record);
				conn.exec_drop("insert into test.extusers (name, surname) values (?, ?);",
											 (&record[0], &record[1])).unwrap();
    }

    Ok(())
}

fn main() {
    // If an error occurs print error
    if let Err(e) = read_from_stdin() {
        eprintln!("{}", e);
    }
}
