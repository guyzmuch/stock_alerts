
mod stock_price_connector;
mod stock_file_io;

use std::env;
use std::collections::HashMap;
use std::error::Error;
use config::{Config, Environment, Value};
use dotenv::dotenv;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize};
use tokio::runtime::Runtime;

use stock_price_connector::StockPriceConnector;

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
    let use_aws: bool = env::var("SA_STOCK_API_USE_AWS")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);
    
    // Initialize the configuration builder
    let builder = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(Environment::with_prefix("sa_stock"))
        .set_override("use_mock_data", use_mock_data)?
        .set_override("use_aws", use_aws)?;
 
    // Build the configuration
    let config = builder.build()?;

    println!("config: \n{:?}", config);

    let stock_file = stock_file_io::stock_type_factory(config.clone());
    let api_connector = StockPriceConnector::new(&config)?;

    if let Some(stocks) = config.get::<HashMap<String, Value>>("stocks").ok() {
        for (stock_reference, stock_name) in stocks {
            if let Some(stock_name_str) = stock_name.into_string().ok() {
                println!("********");
                println!("Stock history for the stock: {}", stock_name_str);
                let mut previous_values = stock_file.read_file(stock_name_str.to_string())?;
                for result in previous_values {
                    println!("result {:?}", result);
                }
                
                // get the latest value
                let rt = Runtime::new().unwrap();
                let (stock_date, stock_value) = rt.block_on(
                    api_connector.get_stock_price(&stock_reference)
                )?;
                
                stock_file.append_to_file(stock_name_str, stock_date, stock_value)?;
            }
        }
    }

    Ok(())
}
