use std::fs::{self, File};
use std::io::Write;
use std::error::Error;
use csv;
use toml::Value;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize};

static CSV_FILE_HEADER: &str = "date,value\n";

#[derive(Debug, Deserialize)]
struct StockHistory {
    date: DateTime<Utc>,
    value: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_as_string = fs::read_to_string("config.toml")?;
    let config: Value = toml::from_str(&config_as_string)?;

    if let Some(stocks) = config.get("stocks").and_then(|s| s.as_table()) {
        for (stock_reference, stock_name) in stocks {
            if let Some(stock_name_str) = stock_name.as_str() {
                println!("********");
                println!("Stock history for the stock: {}", stock_name_str);
                let mut previous_values = read_from_local(stock_name_str.to_string())?;
                for result in previous_values {
                    println!("result {:?}", result);
                }
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

fn read_from_s3() {

}