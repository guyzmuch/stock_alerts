use rand::Rng;
use chrono::Utc;
use config::{Config};

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

  pub async fn get_stock_price(&self, symbol: &str) -> Result<(String, f64), Box<dyn std::error::Error>> {
      if self.use_mock {
        // Create a random number generator
        let mut rng = rand::thread_rng();
        let random_float_in_range: f64 = rng.gen_range(0.1..200.0);
        let now = Utc::now();
        return Ok((now.to_rfc3339(), random_float_in_range));

      }

      let url_for_call = self.api_url
        .replace("{symbol}", symbol)
        .replace("{api_key}", &self.api_key);

      println!("url_for_call {}", url_for_call);

      Ok(0.0)
  }
}
