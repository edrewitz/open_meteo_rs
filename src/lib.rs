mod format_url;
mod retrieve;
mod dataframe;

use pyo3::prelude::*;


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
