// filepath: src/main.rs
mod api;
mod models;
mod utils;

use crate::api::european_data_api::EuropeanDataAPI;
use crate::models::energy_price_collection::EnergyPriceCollection;
use crate::utils::time_utils::{get_time_in_history, get_local_time, set_to_midnight, set_to_end_of_day, format_date};
use chrono::Utc;

const USAGE_TYPE_ELECTRICITY: i32 = 1;
const USAGE_TYPE_GAS: i32 = 3;

#[tokio::main]
async fn main() {
    println!("Energy Price App");

    let from_date = format_date(get_time_in_history(365));
    let till_date = format_date(Utc::now());

    let url = format!("https://api.energyzero.nl/v1/energyprices?&fromDate={}&tillDate={}&interval=4&usageType={}&inclBtw=true", from_date, till_date, USAGE_TYPE_ELECTRICITY);
    let electricity_prices = EuropeanDataAPI::fetch_energy_prices(&url).await.unwrap();

    let mut electricity_collection = EnergyPriceCollection::new();
    for price in electricity_prices {
        electricity_collection.add_price(price);
    }

    electricity_collection.display();
    electricity_collection.write_to_file_per_day("EnergyPrices/electricity/").unwrap();

    let url = format!("https://api.energyzero.nl/v1/energyprices?&fromDate={}&tillDate={}&interval=4&usageType={}&inclBtw=true", from_date, till_date, USAGE_TYPE_GAS);
    let gas_prices = EuropeanDataAPI::fetch_energy_prices(&url).await.unwrap();

    let mut gas_collection = EnergyPriceCollection::new();
    for price in gas_prices {
        gas_collection.add_price(price);
    }

    gas_collection.display();
    gas_collection.write_to_file_per_day("EnergyPrices/gas/").unwrap();
}