
use config::{Config};
use csv;
use std::error::Error;
use crate::stock_file_io::{CSV_FILE_HEADER, StockFileIO, StockHistory};

#[derive(Clone)]
pub struct Aws {
  pub config: Config,
}


impl StockFileIO for Aws {

  fn read_file(self: Self, stock_name: String) ->  Result<Vec<StockHistory>, Box<dyn Error>> {
    let mut stock_histories: Vec<StockHistory> = Vec::new();
    Ok(stock_histories)
  }

  fn append_to_file(self: Self, stock_name: String, stock_date: String, stock_value: f64) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}
  