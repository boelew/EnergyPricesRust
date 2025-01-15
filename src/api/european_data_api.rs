// filepath: src/api/european_data_api.rs
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use crate::models::energy_price::EnergyPrice;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    average: f64,
    Prices: Vec<EnergyPrice>,
}

pub struct EuropeanDataAPI;

impl EuropeanDataAPI {
    pub async fn fetch_energy_prices(url: &str) -> Result<Vec<EnergyPrice>, Error> {
        let response = reqwest::get(url).await?;
        //let raw_json: Value = response.json().await?;

        // Print the raw JSON response
        //println!("Raw API response: {}", raw_json);

        // Deserialize the JSON into ApiResponse
        let api_response: ApiResponse = response.json().await?;
        Ok(api_response.Prices)
    }
}