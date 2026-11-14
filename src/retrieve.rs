/// This file hosts the client that retrieves data from the Open-Meteo API.
/// 
/// (C) Eric J. Drewitz 2026


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

/// Fetches Open-Meteo hourly forecast data and builds a Polars DataFrame dynamically.
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