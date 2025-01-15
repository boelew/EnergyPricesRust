// filepath: src/models/energy_price.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnergyPrice {
    pub price: f64,
    //pub currency: String,
    pub readingDate: String,
}

impl EnergyPrice {
    pub fn display(&self) {
        println!("Price: {} at {}", self.price, self.readingDate);
    }
}
