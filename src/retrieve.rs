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

/// Fetches the latest point weather forecast data for a given model at a given latitude/longitude
/// 
/// Arguments:
/// 1) latitude (f64) - The latitude in decimal degrees. 
///     (+) Values = Northern Hemisphere
///     (-) Values = Southern Hemisphere
/// 
/// 2) Longitude (f64) - The longitude in decimal degrees.
///     (+) Values = Eastern Hemisphere
///     (-) Values = Western Hemisphere
/// 
/// 3) variables (Vec<String>) - The list of variables.
/// 
/// 4) model (&str) - The forecast model.
/// 
/// 5) days (i32) - The amount of days for the forecast to go out. 
/// 
/// 6) proxy (Option<&str>) - Set to None if you are not using a proxy server connection.
///     If you are using a proxy server connection define it here 
/// 
///     Example: proxy:https://proxy-server-address:proxy-server-port
/// 
/// # Examples
/// 
/// ```
/// // Retrieve the GFS 0.25x0.25 Degree 2-Meter Temperature and 2-Meter Relative Humidity Forecasts for 7 days at Islip Mac Arthur Airport
///
/// // Imports
/// use crate::retrieve::fetch_weather_forecast;
/// 
/// // Latitude = 40.79°N
/// let latitude: f64 = 40.79;
/// 
/// // Longitude = 73.10°W
/// let longitude: f64 = -73.10;
/// 
/// // Define our variables
/// let variables: Vec<String> = vec!["temperature_2m".to_string(), 
///                                   "relative_humidity_2m".to_string()];
/// 
/// // Define our model as the GFS 0.25x0.25 Degree
/// let model = "ncep gfs global";
/// 
/// // Define our days as 7 so the forecast spans 7 days
/// let days: i32 = 7;
/// 
/// // Set proxy=None since we are not using a proxy server
/// let proxy = None;
/// 
/// // Make our request and retrieve our weather forecast data
/// 
/// let df = fetch_weather_df(latitude, longitude, variables, model, days, proxy)?;
/// 
/// // Prints our dataframe to the screen
/// println!("{}", df);
/// 
/// ```
///     
pub fn fetch_weather_forecast(
    latitude: f64,
    longitude: f64,
    variables: Vec<String>,
    model: &str,
    days: i32,
    proxy: Option<&str>
                ) -> Result<DataFrame, Box<dyn std::error::Error>> {

    let url = format_string(latitude, longitude, variables, model, days);
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