#[cfg(test)]
mod tests;

mod lib;

#[tokio::main]
async fn main() {
    let origin = "DUB";
    let destination = "STN";

    match lib::get_one_way_flights(origin, destination, "2024-04-18").await {
        Ok(res) => {
            println!("API Response: {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };

    match lib::get_return_flights(origin, destination, "2024-04-18", "2024-04-19").await {
        Ok(res) => {
            println!("API Response: {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };

    match lib::get_cheapest_return_flights_from_weekdays(origin,destination,"2024-04-18", "2024-08-18", "monday", "sunday").await {
        Ok(res) => {
            println!("All Possible Flights:\n {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };
}
