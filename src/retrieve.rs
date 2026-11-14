/// This file hosts the client that retrieves data from the Open-Meteo API.
/// 
/// (C) Eric J. Drewitz 2026

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
pub fn fetch_weather(latitude: f64,
                    longitude: f64,
                    variables: Vec<String>,
                    model: &str,
                    days: i32,
                    proxy: Option<&str>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    
                    
    let joined_vars = variables.join(",");


    let url = format!("https://api.open-meteo.com/v1/forecast?\
    latitude={latitude}&longitude={longitude}&\
    hourly={joined_vars}&\
    models={model}&forecast_days={days}");

    let parsed: ApiResponse = if let Some(proxy_url) = proxy {
        let proxy_obj = Proxy::all(proxy_url)?;
        let client = reqwest::blocking::Client::builder()
            .proxy(proxy_obj)
            .build()?;

        client.get(url).send()?.json()?
    } else {
        reqwest::blocking::get(url)?.json()?
    };
    
    

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

    // Note: Use `DataFrame::new_infer_height(columns)?` if you are on Polars 0.55+
    let mut df = DataFrame::new_infer_height(columns)?;

    // Convert "time" to Datetime if present
    if df.column("time").is_ok() {
        df = df
            .lazy()
            .with_column(
                col("time")
                    .str()
                    .to_datetime(
                        Some(TimeUnit::Milliseconds),
                        None,
                        StrptimeOptions {
                            format: Some("%Y-%m-%dT%H:%M".into()),
                            strict: true,
                            exact: true,
                            cache: true,
                        },
                        lit("raise"),
                    )
                    .alias("time"),
            )
            .collect()?;
    }

    Ok(df)
}