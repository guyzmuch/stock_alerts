
mod stock_price_connector;
use std::env;
use std::{collections::HashMap, fs::{self, File}, io::Write};
use std::error::Error;
use config::{Config, Environment, Value};
use dotenv::dotenv;
use csv;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize};
use tokio::runtime::Runtime;

use stock_price_connector::StockPriceConnector;

static CSV_FILE_HEADER: &str = "date,value\n";

#[derive(Debug, Deserialize)]
struct StockHistory {
    date: DateTime<Utc>,
    value: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from the .env file
    dotenv().ok();
    
    // Read and convert the environment variable to a boolean
    let use_mock_data: bool = env::var("SA_STOCK_API_USE_MOCK_DATA")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);
    
    // Initialize the configuration builder
    let builder = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(Environment::with_prefix("sa_stock"))
        .set_override("use_mock_data", use_mock_data)?;
 
    // Build the configuration
    let config = builder.build()?;

    println!("config: \n{:?}", config);

    let api_connector = StockPriceConnector::new(&config)?;

    if let Some(stocks) = config.get::<HashMap<String, Value>>("stocks").ok() {
        for (stock_reference, stock_name) in stocks {
            if let Some(stock_name_str) = stock_name.into_string().ok() {
                println!("********");
                println!("Stock history for the stock: {}", stock_name_str);
                let mut previous_values = read_from_local(stock_name_str.to_string())?;
                for result in previous_values {
                    println!("result {:?}", result);
            }

            // get the latest value
            let rt = Runtime::new().unwrap();
                let (stock_date, stock_value) = rt.block_on(
                    api_connector.get_stock_price(&stock_reference)
                )?;
                
                write_local(stock_name_str, stock_date, stock_value)?;
            }
        }
    }

    Ok(())
}

fn read_from_local(stock_name: String) -> Result<Vec<StockHistory>, Box<dyn Error>> {
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

fn write_local(stock_name: String, stock_date: String, stock_value: f64) -> Result<(), Box<dyn Error>> {
    // file should have been created in the read_from_local, so no need to verify it here
    let file_path = format!("local/{}.csv", stock_name.to_lowercase());
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(file_path)?;

    file.write_all(format!("{},{}\n", stock_date, stock_value).as_bytes())?;

    Ok(())
}

fn read_from_s3() {

}