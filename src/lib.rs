use serde::{Deserialize};
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;


#[derive(Debug, Deserialize)]
struct Airport {
    #[serde(rename = "countryName")]
    country_name: String,
    #[serde(rename = "iataCode")]
    iata_code: String,
    name: String,
    #[serde(rename = "seoName")]
    seo_name: String,
    city: City,
}

#[derive(Debug, Deserialize)]
struct City {
    name: String,
    code: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct Flight {
    // Add other fields as needed
    outbound: Outbound,
}

#[derive(Debug, Deserialize)]
struct Outbound {
    // Add other fields as needed
    #[serde(rename = "departureAirport")]
    departure_airport: Airport,
    #[serde(rename = "arrivalAirport")]
    arrival_airport: Airport,
    #[serde(rename = "departureDate")]
    departure_date: String,
    #[serde(rename = "arrivalDate")]
    arrival_date: String,
    price: Price,
    #[serde(rename = "flightKey")]
    flight_key: String,
    #[serde(rename = "flightNumber")]
    flight_number: String,
    #[serde(rename = "previousPrice")]
    previous_price: Option<Value>,
    #[serde(rename = "priceUpdated")]
    price_updated: i64,
}

#[derive(Debug, Deserialize)]
struct Price {
    value: f64,
    #[serde(rename = "valueMainUnit")]
    value_main_unit: String,
    #[serde(rename = "valueFractionalUnit")]
    value_fractional_unit: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    #[serde(rename = "currencySymbol")]
    currency_symbol: String,
}

#[derive(Debug, Deserialize)]
struct Fare {
    // Add other fields as needed
    outbound: Outbound,
    summary: Summary,
}

#[derive(Debug, Deserialize)]
struct Summary {
    // Add other fields as needed
    price: Price,
    #[serde(rename = "previousPrice")]
    previous_price: Option<Value>,
}

#[derive(Default, Debug, Deserialize)]
pub struct FlightResponse {
    fares: Vec<Fare>,
}

impl fmt::Display for FlightResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.fares.is_empty() {
            write!(f, "Sorry. No fares are available.");
            return Ok(());
        }
        for fare in &self.fares {
            writeln!(
                f,
                "Fly from {} to {}\nFly out: {}\nArrive: {}\nfor {}{}",
                fare.outbound.departure_airport.seo_name,
                fare.outbound.arrival_airport.seo_name,
                fare.outbound.departure_date,
                fare.outbound.arrival_date,
                fare.summary.price.currency_symbol,
                fare.summary.price.value,
            )?;
        }
        
        Ok(())
    }
}

pub async fn handle_connection(params: HashMap<String,String>) -> Result<FlightResponse, Box<dyn std::error::Error>> {

    let api_url = "https://services-api.ryanair.com/farfnd/v4/oneWayFares";

    // Make the API request
    let response = reqwest::Client::new()
    .get(api_url)
    .query(&params)
    .send()
    .await;
    
    // Check if the request was successful (status code 200)
    let result = match response {
        Ok(res) => {
            if res.status().is_success() {
                // Parse and handle the response
                let body = res.text().await?;
                let result: FlightResponse = serde_json::from_str(&body).expect("Error parsing JSON");
                return Ok(result);
            } else {
                // Handle the error
                println!("Error: {:?}", res.status());
                let empty_flight_response: FlightResponse = Default::default();
                return Ok(empty_flight_response);
            }
        }
        Err(error) => {
            // Handle the error from the request
            println!("Request error: {:?}", error);
            return Err(Box::new(error));
        }
    };

    result
}