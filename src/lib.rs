mod format_url;
mod retrieve;
mod dataframe;

use pyo3::prelude::*;
use polars::prelude::*;
use crate::retrieve::fetch_weather_forecast;

/// Fetches the latest point weather forecast data for a given model at a given latitude/longitude
/// 
/// Arguments:
/// 1) latitude (`f64`) - The latitude in decimal degrees. 
///     (+) Values = Northern Hemisphere
///     (-) Values = Southern Hemisphere
/// 
/// 2) Longitude (`f64`) - The longitude in decimal degrees.
///     (+) Values = Eastern Hemisphere
///     (-) Values = Western Hemisphere
/// 
/// 3) variables (`Vec<String>`) - The list of variables.
/// 
/// 4) model (`&str`) - The forecast model.
/// 
/// 5) days (`i32`) - The amount of days for the forecast to go out. 
/// 
/// 6) proxy (`Option<&str>`) - Set to None if you are not using a proxy server connection.
///     If you are using a proxy server connection define it here 
/// 
///     Example: `proxy:https://proxy-server-address:proxy-server-port`
/// 
/// **Returns**
/// 
/// A Polars DataFrame of the requested data. 
/// 
/// # Examples
/// 
/// ```
/// // Retrieve the latest GFS 0.25x0.25 Degree 2-meter temperature and relative humidity forecast for Islip Mac Arthur Airport
/// 
/// // Imports
/// use openmeteoapi_rs::get_forecast;
/// 
/// // Define latitude
/// let latitude: f64 = 40.79;
/// 
/// // Define longitude
/// let longitude: f64 = -73.10;
/// 
/// // Define variables
/// let variables: Vec<String> = vec![
///     "temperature_2m".to_string(), 
///     "relative_humidity_2m".to_string(),
/// ];
/// 
/// // Define weather forecast model
/// let model = "ncep gfs global";
/// 
/// // Define number of days for the forecast
/// let days: i32 = 7;
/// 
/// // Define a proxy server if using one. 
/// // In this example we are not using a proxy server. 
/// let proxy = None;
/// 
/// // Retrieve the forecast data
/// let df = get_forecast(latitude, longitude, variables, model, days, proxy).unwrap();
/// 
/// // Print the forecast to the console
/// println!("{}", df);
/// ```
pub fn get_forecast(
    latitude: f64,
    longitude: f64,
    variables: Vec<String>,
    model: &str,
    days: i32,
    proxy: Option<&str>
                ) -> Result<DataFrame, Box<dyn std::error::Error>> {

    let df = fetch_weather_forecast(latitude, longitude, variables, model, days, proxy);

    df

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_forecast() {
        let latitude: f64 = 40.79;
        let longitude: f64 = -73.10;
        let variables: Vec<String> = vec![
            "temperature_2m".to_string(),
            "relative_humidity_2m".to_string(),
        ];
        let model = "ncep gfs global";
        let days: i32 = 7;
        let proxy = None;

        // Use .unwrap() instead of ? in a test returning ()
        let df = get_forecast(latitude, longitude, variables, model, days, proxy).unwrap();

        println!("{}", df);
    }

}



// A Python module implemented in Rust.
#[pymodule]
mod _core {

    use pyo3::prelude::*;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3_polars::types::PyDataFrame;
    use crate::retrieve::fetch_weather_forecast;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn get_forecast(
        latitude: f64,
        longitude: f64,
        variables: Vec<String>,
        model: &str,
        days: i32,
        proxy: Option<&str>
    ) -> PyResult<PyDataFrame> {
        
        let df = fetch_weather_forecast(latitude, longitude, variables, model, days, proxy)
                    .map_err(|e| PyRuntimeError::new_err(format!("Request failed: {:#}", e)))?;

    // Wrap the Polars DataFrame so it converts to a Python polars.DataFrame
    Ok(PyDataFrame(df))

    }
}
