#[cfg(test)]
mod tests;

mod lib;

#[tokio::main]
async fn main() {
    let origin = "DUB";
    let destination = "STN";

    match lib::get_one_way_flights(origin, destination, "2024-08-18").await {
        Ok(res) => {
            println!("Direct Flights: {}", res);
        },
        Err(err) => {
            println!("No response for direct flights: {err}");
        }
    };

    match lib::get_return_flights(origin, destination, "2024-08-18", "2024-04-19").await {
        Ok(res) => {
            println!("Return Flights: {}", res);
        },
        Err(err) => {
            println!("No response for return flights: {err}");
        }
    };

    match lib::get_cheapest_return_flights_from_weekdays(origin,destination,"2024-08-17", "2024-09-17", "monday", "sunday").await {
        Ok(res) => {
            println!("All Possible Flights:\n{}", res);
        },
        Err(err) => {
            println!("No response for all return flights: {err}");
        }
    };
}
