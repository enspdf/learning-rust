#![deny(clippy::all)]

use std::env;

const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

struct Manufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or_default().contains(needle)
            || self.common_name.unwrap_or_default().contains(needle)
            || self.country.unwrap_or_default().contains(needle)
    }
}

impl<'a> Manufacturer<'a> {
    fn description(&self) -> String {
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();
        format!(
            "\rName: {}\n\tCommon Name: {}\n\tCountry: {}",
            name, common_name, country
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }

    let keywords = &args[1];

    let client = reqwest::Client::new();
    let res = client
        .get(API_URL)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let manufacturer_json = res
        .as_object()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"Results")
        .unwrap()
        .1
        .as_array()
        .unwrap()
        .iter();

    let manufacturers = manufacturer_json.map(|manufacturer_json| {
        let obj = manufacturer_json.as_object().unwrap();
        // println!("{:?}", obj);
        let country = obj.get("Country").unwrap().as_str();
        let common_name = obj.get("Mfr_CommonName").unwrap().as_str();
        let name = obj.get("Mfr_Name").unwrap().as_str();

        Manufacturer {
            name,
            common_name,
            country,
        }
    });

    let found_manufacturers = manufacturers
        .filter(|manufacturer| manufacturer.contains(keywords))
        .collect::<Vec<_>>();

    if found_manufacturers.is_empty() {
        Err("No manufacturers found".into())
    } else {
        println!("Found {} manufacturers", found_manufacturers.len());
        for (index, man) in found_manufacturers.iter().enumerate() {
            println!("Manufacturer #{}", index + 1);
            println!("{}", man.description());
        }

        Ok(())
    }
}
