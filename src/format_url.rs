/// Formats the URL for the HTTP request
/// 
/// (C) Eric J. Drewitz 2026

use std::collections::HashMap;

fn get_model(model: &str) -> String {
    let models = HashMap::from([
        ("Best Match", "best_match"),
        ("ECMWF IFS HRES 9km", "ecmwf_ifs"),
        ("ECMWF IFS 0.25", "ecmwf_ifs025"),
    ]);

    models
        .get(model)
        .unwrap_or(&"best_match")
        .to_string()
}

pub(crate) fn format_string(
    latitude: f64,
    longitude: f64,
    variables: Vec<String>,
    model: &str,
    days: i32
                ) -> String {

    let joined_vars = variables.join(",");

    let selected_model = get_model(model);

    let url = format!("https://api.open-meteo.com/v1/forecast?\
    latitude={latitude}&longitude={longitude}&\
    hourly={joined_vars}&\
    models={selected_model}&forecast_days={days}");

    url

}