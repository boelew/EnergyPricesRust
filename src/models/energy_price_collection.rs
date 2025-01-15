// filepath: src/models/energy_price_collection.rs
use crate::models::energy_price::EnergyPrice;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub struct EnergyPriceCollection {
    pub prices: Vec<EnergyPrice>,
}

impl EnergyPriceCollection {
    pub fn new() -> Self {
        Self { prices: Vec::new() }
    }

    pub fn add_price(&mut self, price: EnergyPrice) {
        self.prices.push(price);
    }

    pub fn display(&self) {
        for price in &self.prices {
            price.display();
        }
    }

    pub fn write_to_file_per_day(&self, directory: &str) -> io::Result<()> {
        let mut prices_per_day: HashMap<String, Vec<&EnergyPrice>> = HashMap::new();

        for price in &self.prices {
            let day = &price.readingDate[..10]; // Extract the date part (YYYY-MM-DD)
            prices_per_day
                .entry(day.to_string())
                .or_default()
                .push(price);
        }

        // Ensure the directory exists
        fs::create_dir_all(directory)?;

        for (day, day_prices) in prices_per_day {
            let filename = format!("{}/{}_prices.csv", directory, day);
            let path = Path::new(&filename);
            let mut file = File::create(&path)?;

            writeln!(file, "Price,Currency,Timestamp")?;
            for price in day_prices {
                writeln!(file, "{},{}", price.price, price.readingDate)?;
            }
        }

        Ok(())
    }
}
