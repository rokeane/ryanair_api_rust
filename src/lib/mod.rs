mod flight_builder;
use flight_builder::FlightResponse;
use flight_builder::ReturnFlight;
use flight_builder::AllReturnFlights;

use std::collections::HashMap;

use chrono::prelude::*;
use chrono::{NaiveDate, TimeDelta, Weekday};

use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

pub async fn get_one_way_flights(
    source: &str, 
    dest: &str, 
    departure_date: &str, 
) -> Result<FlightResponse, Box<dyn std::error::Error>> {
    let mut params: HashMap<&str,&str> = HashMap::new();
    
    params.insert("departureAirportIataCode", source); 
    params.insert("arrivalAirportIataCode", dest); 
    params.insert("outboundDepartureDateFrom", departure_date); 
    params.insert("outboundDepartureDateTo", departure_date); 
    params.insert("currency", "EUR"); 

    let api_url = "https://services-api.ryanair.com/farfnd/v4/oneWayFares";

    let response = reqwest::Client::new()
    .get(api_url)
    .query(&params)
    .send()
    .await;
    
    let _ = match response {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.text().await?;
                let result: FlightResponse = serde_json::from_str(&body).expect("Error parsing JSON");
                return Ok(result);
            } else {
                println!("Error: {:?}", res.status());
                let empty_flight_response: FlightResponse = Default::default();
                return Ok(empty_flight_response);
            }
        }
        Err(error) => {
            println!("Request error: {:?}", error);
            return Err(Box::new(error));
        }
    };

}


pub async fn get_return_flights(
    source: &str, 
    dest: &str, 
    departure_date: &str, 
    return_date: &str, 
) -> Result<AllReturnFlights, Box<dyn std::error::Error>> {

    let flights_to_dest = get_one_way_flights(source, dest, departure_date).await?.fares;

    let flights_from_dest = get_one_way_flights(dest, source, return_date).await?.fares;
    
    let mut result :AllReturnFlights = Default::default();

    if flights_to_dest.is_empty() || flights_from_dest.is_empty() {
        return Ok(result);
    }

    for to_dest in &flights_to_dest {
        for from_dest in &flights_from_dest {
            let total_price = from_dest.summary.price.clone() + to_dest.summary.price.clone(); 
            let return_flight = ReturnFlight{to_destination: to_dest.clone(), from_destination: from_dest.clone(), price: total_price};
            result.flights.push(return_flight);
        }
    }

    return Ok(result);
}


// return the cheapest return flights departing day_from and returning on day_to
pub async fn get_cheapest_return_flights_from_weekdays(
    source: &str,
    dest: &str,
    from: &str,
    to: &str,
    day_from: &str,
    day_to: &str,
) -> Result<AllReturnFlights, Box<dyn std::error::Error>> {

    let res = Arc::new(Mutex::new(Vec::new()));
    let dates = get_weekday_combinations(from, to, day_from, day_to);

    // limit the number of api calls that can be spawned simultaneously
    let sem = Arc::new(Semaphore::new(25));

    let mut handles = vec![];
    for (departure_date, return_date) in dates {
        let res = res.clone();
        let source = source.to_owned();
        let dest = dest.to_owned();
        let sem_clone = sem.clone();

        handles.push(tokio::spawn(async move {
            let permit = sem_clone.acquire().await;

            let mut return_flights = get_return_flights(&source.clone(), &dest.clone(), &departure_date, &return_date).await.unwrap().flights;
            let mut shared_data = res.lock().unwrap();
            shared_data.append(&mut return_flights);

            drop(permit);
        }));
    }

    futures::future::join_all(handles).await;

    let mut res_fares = res.clone().lock().unwrap().clone();
    res_fares.sort_by_key(|fare| fare.price.clone());

    return Ok(AllReturnFlights { flights: res_fares });
}

pub fn get_weekday_combinations(
    start_date_str: &str,
    end_date_str: &str,
    start_weekday_str: &str,
    end_weekday_str: &str,
) -> Vec<(String, String)> {

    let mut combinations: Vec<(String, String)> = vec![];

    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d").unwrap();

    let start_weekday = start_weekday_str.parse::<Weekday>().unwrap();
    let end_weekday = end_weekday_str.parse::<Weekday>().unwrap();

    let current_weekday = start_date.weekday();
    
    let days_until_start_weekday = (day_to_int(&start_weekday) - day_to_int(&current_weekday) + 7) % 7;
    let days_until_end_weekday = (day_to_int(&end_weekday) - day_to_int(&current_weekday) + 7) % 7;

    let mut current_start_date = start_date + TimeDelta::days(days_until_start_weekday as i64);
    let mut current_end_date = start_date + TimeDelta::days(days_until_end_weekday as i64);

    if current_end_date < current_start_date {
        current_end_date = current_end_date + TimeDelta::days(7);
    }

    while current_end_date < end_date {
        combinations.push((current_start_date.to_string(), current_end_date.to_string()));
        current_start_date = current_start_date + TimeDelta::days(7);
        current_end_date = current_end_date + TimeDelta::days(7);
    }

    combinations

}

fn day_to_int(day: &chrono::Weekday) -> i8 {
    match day {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
    }
}