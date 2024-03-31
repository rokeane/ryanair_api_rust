mod lib;

#[tokio::main]
async fn main() {
    let origin = "DUB";
    let destination = "STN";
    let inbound_departure_date_from = "2024-04-18";
    let inbound_departure_date_to = "2024-04-19";

    let _response_direct = match lib::get_one_way_flights(origin, destination, inbound_departure_date_from).await {
        Ok(res) => {
            println!("API Response: {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };

    let _response_return = match lib::get_return_flights(origin, destination, inbound_departure_date_from, inbound_departure_date_to).await {
        Ok(res) => {
            println!("API Response: {}", res);
        },
        Err(err) => {
            println!("No tengo respuesta: {err}");
        }
    };

    let ans = lib::get_weekday_combinations("2024-04-18", "2024-06-18", "monday", "sunday");
    for i in ans {
        let (date_from, date_to) = i;
        println!("From: {} to {}", date_from, date_to);
    }

}
