use serde::{Deserialize};
use std::cmp::Ordering;
use serde_json::Value;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
struct City {
    name: String,
    code: String,
    #[serde(rename = "countryCode")]
    country_code: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Flight {
    outbound: Outbound,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Outbound {
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

#[derive(Clone, Debug, Deserialize)]
pub struct Price {
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

impl Eq for Price {}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            return Ordering::Less
        } else if self.value > other.value {
            return Ordering::Greater
        }
        Ordering::Equal
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Fare {
    pub outbound: Outbound,
    pub summary: Summary,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Summary {
    pub price: Price,
    #[serde(rename = "previousPrice")]
    previous_price: Option<Value>,
}

#[derive(Default, Debug, Deserialize)]
pub struct FlightResponse {
    pub fares: Vec<Fare>,
}

impl fmt::Display for FlightResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.fares.is_empty() {
            return write!(f, "Sorry. No fares are available.");
        }
        for fare in &self.fares {
            writeln!(
                f,
                "\nFly from {} to {}\nFly out: {}\nArrive: {}\nfor {}{}",
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