//! # openmeteoapi_rs
//! ***(C) Eric J. Drewitz 2026***
//! 
//! A Rust interface for retrieving data from Open-Meteo API. 
//! This package also has Python bindings.



mod format_url;
mod retrieve;
mod dataframe;

#[cfg(feature = "python")]
use pyo3::prelude::*;

use polars::prelude::*;
use crate::retrieve::fetch_hourly_weather_forecast;

/// Fetches the latest hourly point weather forecast data for a given model at a given latitude/longitude
/// 
/// **Arguments**:
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
/// *Variables*
/// 
/// - "temperature_2m"
/// - "relative_humidity_2m"
/// - "dew_point_2m"
/// - "apparent_temperature"
/// - "precipitation_probability"
/// - "precipitation"
/// - "rain"
/// - "showers"
/// - "snowfall"
/// - "snow_depth"
/// - "weather_code"
/// - "surface_pressure"
/// - "pressure_msl"
/// - "cloud_cover"
/// - "cloud_cover_low"
/// - "cloud_cover_mid"
/// - "cloud_cover_high"
/// - "visibility"
/// - "evapotranspiration"
/// - "et0_fao_evapotranspiration"
/// - "vapour_pressure_deficit"
/// - "wind_speed_10m"
/// - "wind_speed_120m"
/// - "wind_speed_80m"
/// - "wind_speed_180m"
/// - "wind_direction_10m"
/// - "wind_direction_80m"
/// - "wind_direction_120m"
/// - "wind_direction_180m"
/// - "wind_gusts_10m"
/// - "temperature_80m"
/// - "temperature_120m"
/// - "temperature_180m"
/// - "soil_temperature_0cm"
/// - "soil_temperature_6cm"
/// - "soil_temperature_18cm"
/// - "soil_temperature_54cm"
/// - "soil_moisture_0_to_1cm"
/// - "soil_moisture_3_to_9cm"
/// - "soil_moisture_1_to_3cm"
/// - "soil_moisture_9_to_27cm"
/// - "soil_moisture_27_to_81cm"
/// - "uv_index"
/// - "uv_index_clear_sky"
/// - "is_day"
/// - "sunshine_duration"
/// - "wet_bulb_temperature_2m"
/// - "total_column_integrated_water_vapour"
/// - "cape"
/// - "lifted_index"
/// - "convective_inhibition"
/// - "freezing_level_height"
/// - "boundary_layer_height"
/// 
/// 4) model (`&str`) - The forecast model.
/// 
/// *Models*
/// 
/// - "best match"
/// - "ecmwf ifs hres 9km"
/// - "ecmwf ifs 0p25"
/// - "ecmwf aifs 0p25 single"
/// - "cma grapes global"
/// - "dwd icon seamless"
/// - "dwd icon global native"
/// - "dwd aicon global native"
/// - "dwd icon eu native"
/// - "dwd icon d2 native"
/// - "dwd icon d2 ruc native"
/// - "met norway seamless"
/// - "met norway nordic"
/// - "meteoswiss icon seamless"
/// - "meteoswiss icon ch1"
/// - "meteoswiss icon ch2"
/// - "ncep gfs seamless"
/// - "ncep gfs global"
/// - "ncep rrfs north america"
/// - "ncep rrfs conus"
/// - "ncep nbm conus"
/// - "ncep aigfs 0p25"
/// - "ncep hgefs 0p25 ensemble mean"
/// - "gem seamless"
/// - "cmc gem global"
/// - "cmc gem regional"
/// - "cmc gem hrdps continental"
/// - "cmc gem hrdps west"
/// - "knmi seamless"
/// - "knmi harmonie arome europe"
/// - "knmi harmonie arome netherlands"
/// - "geosphere seamless"
/// - "geosphere claef austria"
/// - "jma seamless"
/// - "jma msm"
/// - "jma gsm"
/// - "meteofrance seamless"
/// - "meteofrance arpege world"
/// - "meteofrance arpege europe"
/// - "meteofrance arome france"
/// - "meteofrance arome france hd"
/// - "dmi seamless"
/// - "dmi harmonie arome europe"
/// - "chmi aladin seamless"
/// - "chmi aladin central europe 2km"
/// - "chmi aladin cz 1km"
/// - "italia meteo arpae icon 2i"
/// - "uk met office seamless"
/// - "uk met office global 10km"
/// - "uk met office uk 2km"
/// 
/// 5) days (`i32`) - The amount of days for the forecast to go out. 
/// 
/// 6) wind_speed_units (`&str`) - The units for wind speed.
/// 
/// *Wind Speed Units*
///
/// - "kilometers per hour"
/// - "meters per second"
/// - "miles per hour"
/// - "knots"
/// 
/// 7) temperature_units (`&str`) - The units for temperature.
/// 
/// *Temperature Units*
/// 
///  - "fahrenheit"
///  - "celsius" 
/// 
/// 8) precipitation_units (`&str`) - The units for precipitation.
/// 
/// *Precipitation Units*
/// 
///  - "inch"
///  - "millimeter"
/// 
/// 9) proxy (`Option<&str>`) - Set to None if you are not using a proxy server connection.
///     If you are using a proxy server connection define it here 
/// 
///     Example: `proxy:https://proxy-server-address:proxy-server-port`
/// 
/// **Returns**
/// 
/// A Polars DataFrame of the requested hourly weather forecast data. 
/// 
/// **Panics**
/// 
/// 1) The client cannot establish a connection to the server.
/// - Server is down and/or having issues.
/// - There is an issue with the local PC's internet connection.
/// 
/// 2) User submits invalid request.
/// - User selects an invalid parameter for a given model.
/// 
/// # Example
/// 
/// ```
/// // Retrieve the latest GFS 0.25x0.25 Degree Hourly Forecast
/// // Location: Islip Mac Arthur Airport (40.79°N/73.10°W)
/// // Variables: 
/// //      1) 2-meter temperature
/// //      2) 2-meter relative humidity
/// //      3) precipitation
/// //      4) 10-meter wind speed
/// // model: GFS 0.25x0.25 Degree
/// // days: 7-Day Forecast
/// // Wind Speed Units: Miles Per Hour
/// // Temperature Units: Fahrenheit
/// // Precipitation Units: Inches
/// // proxy: No Proxy
/// 
/// // Imports
/// use openmeteoapi_rs::get_hourly_forecast;
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
///     "precipitation".to_string(),
///     "wind_speed_10m".to_string(),
/// ];
/// 
/// // Define weather forecast model
/// let model: &str = "ncep gfs global";
/// 
/// // Define number of days for the forecast
/// let days: i32 = 7;
/// 
/// // Define our wind speed units to be miles per hour
/// let wind_speed_units: &str = "miles per hour";
/// 
/// // Define our temperature units
/// let temperature_units: &str = "fahrenheit";
/// 
/// // Define our precipitation units
/// let precipitation_units: &str = "inch";
/// 
/// // Define a proxy server if using one. 
/// // In this example we are not using a proxy server. 
/// let proxy: Option<&str> = None;
/// 
/// // Retrieve the forecast data
/// let df = get_hourly_forecast(
/// latitude, 
/// longitude, 
/// variables, 
/// model, 
/// days, 
/// wind_speed_units,
/// temperature_units,
/// precipitation_units,
/// proxy).unwrap();
/// 
/// // Print the forecast to the console
/// println!("{}", df);
/// ```
pub fn get_hourly_forecast(
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

    let df = fetch_hourly_weather_forecast(
        latitude, 
        longitude, 
        variables, 
        model, 
        days, 
        wind_speed_units,
        temperature_units,
        precipitation_units,
        proxy
    );

    df

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hourly_forecast() {
        let latitude: f64 = 40.79;
        let longitude: f64 = -73.10;
        let variables: Vec<String> = vec![
            "temperature_2m".to_string(),
            "relative_humidity_2m".to_string(),
            "precipitation".to_string(),
            "wind_speed_10m".to_string(),
        ];
        let model: &str = "ncep gfs global";
        let days: i32 = 7;
        let wind_speed_units: &str = "miles per hour";
        let temperature_units: &str = "fahrenheit";
        let precipitation_units: &str = "inch";
        let proxy: Option<&str> = None;

        // Use .unwrap() instead of ? in a test returning ()
        let df = get_hourly_forecast(
            latitude, 
            longitude, 
            variables, 
            model, 
            days, 
            wind_speed_units,
            temperature_units,
            precipitation_units,
            proxy
        ).unwrap();

        println!("{}", df);
    }

}



// Python Bindings
#[cfg(feature = "python")]
#[pymodule]
mod _core {

    use pyo3::prelude::*;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3_polars::types::PyDataFrame;
    use crate::retrieve::fetch_hourly_weather_forecast;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn get_hourly_forecast(
        latitude: f64,
        longitude: f64,
        variables: Vec<String>,
        model: &str,
        days: i32,
        wind_speed_units: &str,
        temperature_units: &str,
        precipitation_units: &str,
        proxy: Option<&str>
    ) -> PyResult<PyDataFrame> {
        
        let df = fetch_hourly_weather_forecast(
            latitude, 
            longitude, 
            variables, 
            model, 
            days, 
            wind_speed_units,
            temperature_units,
            precipitation_units,
            proxy)
                    .map_err(|e| PyRuntimeError::new_err(format!("Request failed: {:#}", e)))?;

    // Wrap the Polars DataFrame so it converts to a Python polars.DataFrame
    Ok(PyDataFrame(df))

    }
}


