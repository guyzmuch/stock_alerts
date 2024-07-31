use config::{Config};

pub struct StockPriceConnector {
  api_url: String,
  api_key: String,
}

impl StockPriceConnector {
  // Create a new SQLite connection
  pub fn new(config: &Config) -> Result<Self, String> {
    let api_url: String = config.get("api_url").unwrap_or("".to_string());
    let api_key: String = config.get("api_key").unwrap_or("".to_string());

    if api_url == "" || api_key == "" {
      return Err("Invalid information for the stock api".to_string())
    }

    Ok(Self {
      api_url,
      api_key,
    })
  }

  pub async fn get_stock_price(&self, symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
      let url_for_call = self.api_url
        .replace("{symbol}", symbol)
        .replace("{api_key}", &self.api_key);

      println!("url_for_call {}", url_for_call);

      Ok(0.0)
  }
}
