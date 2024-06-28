#[cfg(test)]
mod tests;

use ryanair_api;

#[tokio::main]
async fn main() {
    let origin = "DUB";
    let destination = "STN";

    match ryanair_api::get_one_way_flights(origin, destination, "2024-08-18").await {
        Ok(res) => {
            println!("Direct Flights: {}", res);
        }
        Err(err) => {
            println!("Bad response for direct flights: {err}");
        }
    };

    match ryanair_api::get_return_flights(origin, destination, "2024-08-18", "2024-09-18").await {
        Ok(res) => {
            println!("Return Flights: {}", res);
        }
        Err(err) => {
            println!("Bad response for return flights: {err}");
        }
    };

    let cheapest_flights = ryanair_api::get_cheapest_return_flights_from_weekdays(
        origin,
        destination,
        "2024-08-17",
        "2024-09-17",
        "monday",
        "sunday",
    )
    .await;

    println!("Cheapest Return Flights {cheapest_flights}");
}
