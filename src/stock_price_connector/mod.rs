use rand::Rng;
use chrono::{NaiveDate, Utc, DateTime};
use chrono::offset::TimeZone;
use reqwest;
use serde::{Deserialize};
use config::{Config};

use serde::de::{self, Deserializer};
use std::str::FromStr;

fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<T>().map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
struct AlphaVantageErrorResponse {
    #[serde(rename = "Error Message")]
    error_message: String,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageInformationResponse {
    #[serde(rename = "Information")]
    information_message: String,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Debug, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "03. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "04. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "05. price", deserialize_with = "from_str")]
    price: f64,
    #[serde(rename = "06. volume", deserialize_with = "from_str")]
    volume: u64,
    #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    #[serde(rename = "08. previous close", deserialize_with = "from_str")]
    previous_close: f64,
    #[serde(rename = "09. change", deserialize_with = "from_str")]
    change: f64,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AlphaVantageResult {
    Response(AlphaVantageResponse),
    Information(AlphaVantageInformationResponse),
    Error(AlphaVantageErrorResponse),
}


pub struct StockPriceConnector {
  use_mock: bool,
  api_url: String,
  api_key: String,
}

impl StockPriceConnector {
  // Create a new SQLite connection
  pub fn new(config: &Config) -> Result<Self, String> {
    let use_mock: bool = config.get("use_mock_data").unwrap_or(false);
    let api_url: String = config.get("api_url").unwrap_or("".to_string());
    let api_key: String = config.get("api_key").unwrap_or("".to_string());

    if api_url == "" || api_key == "" {
      return Err("Invalid information for the stock api".to_string())
    }

    Ok(Self {
      use_mock,
      api_url,
      api_key,
    })
  }

  
  /* List of errors response from alpha Vantage API (because HTTP code is 200 even for errors)
  rawResponse {
      "Information": "Thank you for using Alpha Vantage! Our standard API rate limit is 25 requests per day. Please subscribe to any of the premium plans at https://www.alphavantage.co/premium/ to instantly remove all daily rate limits."
  } 
  rawResponse {
      "Error Message": "the parameter apikey is invalid or missing. Please claim your free API key on (https://www.alphavantage.co/support/#api-key). It should take less than 20 seconds."
  }
  rawResponse {
    "Error Message": "Invalid API call. Please retry or visit the documentation (https://www.alphavantage.co/documentation/) for GLOBAL_QUOTE."
  }
  */

  pub async fn get_stock_price(&self, symbol: &str) -> Result<(String, f64), Box<dyn std::error::Error>> {
      if self.use_mock {
        // Create a random number generator
        let mut rng = rand::thread_rng();
        let random_float_in_range: f64 = rng.gen_range(0.1..200.0);
        let rounded_number_as_string = format!("{:.5}", random_float_in_range);
        let rounded_number = rounded_number_as_string.parse::<f64>().unwrap();
        let now = Utc::now();
        return Ok((now.to_rfc3339(), rounded_number));

      }

      let url_for_call = self.api_url
        .replace("{symbol}", symbol)
        .replace("{api_key}", &self.api_key);

      match reqwest::get(&url_for_call).await {
        Ok(response) => {
          let raw_response = response.text().await?;

          match serde_json::from_str(&raw_response)? {
            AlphaVantageResult::Response(response_json) => {  
              let parsing_latest_trading_day = NaiveDate::parse_from_str(&response_json.global_quote.latest_trading_day, "%Y-%m-%d")?;
              let latest_trading_day = Utc.from_utc_datetime(&parsing_latest_trading_day.and_hms_opt(17, 0, 0).ok_or("Invalid time")?);
              let formated_latest_trading_day = latest_trading_day.to_rfc3339();
              Ok((formated_latest_trading_day, response_json.global_quote.price))
            },
            AlphaVantageResult::Information(info_response) =>  {
              Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API rate limit reached")))
            },
            AlphaVantageResult::Error(error_response) => {
              Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid API key")))
            },
            _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error deserializing the request")))
          }
        },
        Err(_) => {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch the stock information")));
        }
      }
          
  }
}
