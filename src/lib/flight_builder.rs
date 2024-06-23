use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Debug, Deserialize)]
struct Airport {
    #[allow(dead_code)]
    #[serde(rename = "countryName")]
    country_name: String,
    #[allow(dead_code)]
    #[serde(rename = "iataCode")]
    iata_code: String,
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "seoName")]
    seo_name: String,
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

impl Add for Price {
    type Output = Self;
    fn add(self, other: Self) -> Self {

        Self {
            value: self.value + other.value,
            value_main_unit: self.value_main_unit,
            value_fractional_unit: self.value_fractional_unit,
            currency_code: self.currency_code,
            currency_symbol: self.currency_symbol,
        }
    }
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
}

#[derive(Default, Debug, Deserialize)]
pub struct FlightResponse {
    pub fares: Vec<Fare>,
}

#[derive(Clone)]
pub struct ReturnFlight {
    pub to_destination: Fare,
    pub from_destination: Fare,
    pub price: Price,
}

#[derive(Default)]
pub struct AllReturnFlights {
    pub flights: Vec<ReturnFlight>,
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

impl fmt::Display for AllReturnFlights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.flights.is_empty() {
            return write!(f, "Sorry. No fares are available.");
        }
        let mut n = 1;
        for fare in &self.flights {
            writeln!(
                f,
                "{}.\nFly from {} to {}\nFly out: {}\nArrive: {}\nfor {}{}\n",
                n,
                fare.to_destination.outbound.departure_airport.seo_name,
                fare.to_destination.outbound.arrival_airport.seo_name,
                fare.to_destination.outbound.departure_date,
                fare.to_destination.outbound.arrival_date,
                fare.to_destination.summary.price.currency_symbol,
                fare.to_destination.summary.price.value,
            )?;

            writeln!(
                f,
                "Fly back from {} to {}\nFly out: {}\nArrive: {}\nfor {}{}\n",
                fare.from_destination.outbound.departure_airport.seo_name,
                fare.from_destination.outbound.arrival_airport.seo_name,
                fare.from_destination.outbound.departure_date,
                fare.from_destination.outbound.arrival_date,
                fare.from_destination.summary.price.currency_symbol,
                fare.from_destination.summary.price.value,
            )?;

            writeln!(
                f,
                "For the total price of {}{}\n",
                fare.price.currency_symbol,
                fare.price.value,
            )?;
            

            n = n + 1;
        }
        
        Ok(())
    }
}