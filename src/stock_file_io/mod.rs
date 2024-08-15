
pub static CSV_FILE_HEADER: &str = "date,value\n";

mod local;
mod aws;

use config::Config;
use serde::{self, Deserialize};
use chrono::{DateTime, Utc};
use std::error::Error;


#[derive(Debug, Deserialize)]
pub struct StockHistory {
    date: DateTime<Utc>,
    value: f32,
}

pub trait StockFileIO {
  fn read_file(self: Self, stock_name: String) ->  Result<Vec<StockHistory>, Box<dyn Error>>;
  fn append_to_file(self: Self, stock_name: String, stock_date: String, stock_value: f64) -> Result<(), Box<dyn Error>>;
}

pub fn stock_type_factory(config: Config) -> Box<dyn StockFileIO> {
  let use_aws: bool = config.get("use_aws").unwrap_or(false);

  if use_aws {
    Box::new(aws::Aws { config: config.clone() })
  } else {
    Box::new(local::Local { config: config.clone() })
  }

}
