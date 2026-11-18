// This file hosts the client that retrieves data from the Open-Meteo API.
// 
// (C) Eric J. Drewitz 2026


use crate::format_url::format_string;
use crate::dataframe::drop_empty_cols_and_rows;
use crate::dataframe::format_dataframe;

use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use reqwest::Proxy;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    hourly: BTreeMap<String, Value>,
}

// Fetches the latest hourly point weather forecast data for a given model at a given latitude/longitude
// 
// Arguments:
// 1) latitude (f64) - The latitude in decimal degrees. 
//     (+) Values = Northern Hemisphere
//     (-) Values = Southern Hemisphere
// 
// 2) Longitude (f64) - The longitude in decimal degrees.
//     (+) Values = Eastern Hemisphere
//     (-) Values = Western Hemisphere
// 
// 3) variables (Vec<String>) - The list of variables.
// 
// 4) model (&str) - The forecast model.
// 
// 5) days (i32) - The amount of days for the forecast to go out. 
// 
// 6) proxy (Option<&str>) - Set to None if you are not using a proxy server connection.
//     If you are using a proxy server connection define it here 
// 
//     Example: proxy:https://proxy-server-address:proxy-server-port
// 
// **Returns**
// 
// A Polars DataFrame of the requested data. 
pub fn fetch_hourly_weather_forecast(
    latitude: f64,
    longitude: f64,
    variables: Vec<String>,
    model: &str,
    days: i32,
    wind_speed_units: &str,
    temperature_units: &str,
    precipitation_units: &str,
    proxy: Option<&str>
                ) -> Result<DataFrame, Box<dyn std::error::Error>> {

    let url = format_string(latitude, 
        longitude, 
        variables, 
        model, 
        days,
        wind_speed_units,
        temperature_units,
        precipitation_units
    );
    let client = if let Some(proxy_url) = proxy {
        let proxy_obj = Proxy::all(proxy_url)?;
        reqwest::blocking::Client::builder()
            .proxy(proxy_obj)
            .build()?
    } else {
        reqwest::blocking::Client::new()
    };

    let response = client
        .get(url)
        .send()?
        .error_for_status()?; // Returns Err(reqwest::Error) on 4xx or 5xx

    let parsed: ApiResponse = response.json()?;
    
    let mut columns: Vec<Column> = Vec::new();

    for (key, val) in parsed.hourly {
        if let Some(arr) = val.as_array() {
            if key == "time" {
                let times: Vec<Option<String>> = arr
                    .iter()
                    .map(|v| v.as_str().map(String::from))
                    .collect();
                columns.push(Column::new(key.into(), times));
            } else {
                let values: Vec<Option<f64>> = arr
                    .iter()
                    .map(|v| v.as_f64())
                    .collect();
                columns.push(Column::new(key.into(), values));
            }
        }
    }

    let mut df = format_dataframe(columns)?;

    df = drop_empty_cols_and_rows(&df)?;

    Ok(df)
}