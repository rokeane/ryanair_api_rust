mod flight_builder;
use crate::lib::flight_builder::FlightResponse;

use std::collections::HashMap;

use chrono::prelude::*;
use chrono::{NaiveDate, TimeDelta, Weekday};
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

pub async fn get_one_way_flights(
    source: &str, 
    dest: &str, 
    from: &str, 
) -> Result<FlightResponse, Box<dyn std::error::Error>> {
    let mut params: HashMap<&str,&str> = HashMap::new();
    
    params.insert("departureAirportIataCode", source); 
    params.insert("arrivalAirportIataCode", dest); 
    params.insert("outboundDepartureDateFrom", from); 
    params.insert("outboundDepartureDateTo", from); 

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
    from: &str, 
    to: &str, 
) -> Result<FlightResponse, Box<dyn std::error::Error>> {

    let mut res = Vec::new();

    let mut inbound = get_one_way_flights(source, dest, from).await?.fares;

    let mut outbound = get_one_way_flights(dest, source, to).await?.fares;

    res.append(&mut inbound);
    res.append(&mut outbound);

    return Ok(FlightResponse { fares: res });
}


// return the cheapest return flights departing day_from and returning on day_to
pub async fn get_cheapest_return_flights_from_weekdays(
    source: &str,
    dest: &str,
    from: &str,
    to: &str,
    day_from: &str,
    day_to: &str,
) -> Result<FlightResponse, Box<dyn std::error::Error>> {

    let res = Arc::new(Mutex::new(Vec::new()));
    let dates = get_weekday_combinations(from, to, day_from, day_to);

    // limit the number of api calls that can be spawned simultaneously
    let sem = Arc::new(Semaphore::new(25));

    let mut handles = vec![];
    for (outbound, inbound) in dates {
        let res = res.clone();
        let source = source.to_owned();
        let dest = dest.to_owned();
        let sem_clone = sem.clone();

        handles.push(tokio::spawn(async move {
            let permit = sem_clone.acquire().await;

            let return_flight = get_return_flights(&source.clone(), &dest.clone(), &inbound, &outbound).await;
            match return_flight {
                Ok(ans) => {
                    let mut fares = ans.fares;
                    let mut shared_data = res.lock().unwrap();
                    shared_data.append(&mut fares);
                }
                Err(error) => {
                    println!("Failed to get flights for [{inbound} -> {outbound}]\n due to {:?}", error);
                }
            }
            drop(permit);
        }));
    }

    futures::future::join_all(handles).await;

    let mut res_fares = res.clone().lock().unwrap().clone();
    res_fares.sort_by_key(|fare| fare.summary.price.clone());

    return Ok(FlightResponse { fares: res_fares });
}

pub fn get_weekday_combinations(
    from: &str,
    to: &str,
    day_from: &str,
    day_to: &str,
) -> Vec<(String, String)> {

    let mut res: Vec<(String, String)> = vec![];

    let date_from = NaiveDate::parse_from_str(from, "%Y-%m-%d").unwrap();
    let date_to = NaiveDate::parse_from_str(to, "%Y-%m-%d").unwrap();

    let weekday_date_from = date_from.weekday();
    let weekday_from = day_from.parse::<Weekday>().unwrap();
    let weekday_to = day_to.parse::<Weekday>().unwrap();

    let diff_to_first_date_outbound = (day_to_int(&weekday_from) - day_to_int(&weekday_date_from) + 7) % 7;
    let diff_to_first_date_inbound = (day_to_int(&weekday_to) - day_to_int(&weekday_date_from) + 7) % 7;

    let mut date_from_day_from = date_from + TimeDelta::try_days(diff_to_first_date_outbound as i64).unwrap();
    let mut date_from_day_to = date_from + TimeDelta::try_days(diff_to_first_date_inbound as i64).unwrap();

    while date_from_day_to < date_to {
        res.push((date_from_day_from.to_string(), date_from_day_to.to_string()));
        date_from_day_from = date_from_day_from + TimeDelta::try_days(7).unwrap();
        date_from_day_to = date_from_day_to + TimeDelta::try_days(7).unwrap();
    }

    return res;

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