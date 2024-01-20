// building a REST API in rust
use std::collections::HashMap;

mod lib;

#[tokio::main]
async fn main() {
    // Set your parameters
    let origin = "DUB";
    let destination = "STN";
    let outbound_departure_date_from = "2024-01-20";
    let outbound_departure_date_to = "2024-01-30";
    let outbound_departure_time_from = "10:00";
    let outbound_departure_time_to = "18:00";
    let inbound_departure_time_from = "12:00";
    let inbound_departure_time_to = "20:00";

    // Create a HashMap for parameters
    let mut params = HashMap::new();
    params.insert("departureAirportIataCode".to_string(), origin.to_string());
    params.insert("arrivalAirportIataCode".to_string(), destination.to_string());

    params.insert("outboundDepartureDateFrom".to_string(), outbound_departure_date_from.to_string());
    params.insert("outboundDepartureDateTo".to_string(), outbound_departure_date_to.to_string());

    params.insert("outboundDepartureTimeFrom".to_string(), outbound_departure_time_from.to_string());
    params.insert("outboundDepartureTimeTo".to_string(), outbound_departure_time_to.to_string());

    params.insert("inboundDepartureTimeFrom".to_string(), inbound_departure_time_from.to_string());
    params.insert("inboundDepartureTimeTo".to_string(), inbound_departure_time_to.to_string());

    
    let _response = match lib::handle_connection(params).await {
        Ok(res) => {
            println!("API Response: {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };

}
