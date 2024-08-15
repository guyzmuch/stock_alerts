

use std::{fs::{self, File}, io::Write};
use csv;
use std::error::Error;
use config::{Config};

use crate::stock_file_io::{CSV_FILE_HEADER, StockFileIO, StockHistory};

#[derive(Clone)]
pub struct Local {
  pub config: Config,
}


impl StockFileIO for Local {
  fn read_file(self: Self, stock_name: String) ->  Result<Vec<StockHistory>, Box<dyn Error>> {
    let file_path = format!("local/{}.csv", stock_name.to_lowercase());
 
    let file_content = match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            // create the file if it does not exist
            match File::create(&file_path) {
                Ok(mut file) => {
                    file.write_all(CSV_FILE_HEADER.as_bytes())?;
                    file
                },
                Err(err) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create file")));
                }
            }
        }
    };

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_content);

    let mut stock_histories: Vec<StockHistory> = Vec::new();

    for result in rdr.deserialize() {
        let record: StockHistory = result?;
        stock_histories.push(record);
    }

    Ok(stock_histories)
  }

  fn append_to_file(self: Self, stock_name: String, stock_date: String, stock_value: f64) -> Result<(), Box<dyn Error>> {
    // file should have been created in the read_from_local, so no need to verify it here
    let file_path = format!("local/{}.csv", stock_name.to_lowercase());
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(file_path)?;

    file.write_all(format!("{},{}\n", stock_date, stock_value).as_bytes())?;

    Ok(())
  }
}
