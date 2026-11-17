// Formats the URL for the HTTP request
// 
// (C) Eric J. Drewitz 2026

use std::collections::HashMap;

// Returns the URL format for the model portion of the API query
fn get_model(model: &str) -> String {
    let models = HashMap::from([
        ("best match", "best_match"),
        ("ecmwf ifs hres 9km", "ecmwf_ifs"),
        ("ecmwf ifs 0p25", "ecmwf_ifs025"),
        ("ecmwf aifs 0p25 single", "ecmwf_aifs025_single"),
        ("cma grapes global", "cma_grapes_global"),
        ("dwd icon seamless", "dwd_icon_seamless"),
        ("dwd icon global native", "dwd_icon_global_native"),
        ("dwd aicon global native", "dwd_aicon_global_native"),
        ("dwd icon eu native", "dwd_icon_eu_native"),
        ("dwd icon d2 native", "dwd_icon_d2_native"),
        ("dwd icon d2 ruc native", "dwd_icon_d2_ruc_native"),
        ("met norway seamless", "metno_seamless"),
        ("met norway nordic", "metno_nordic"),
        ("meteoswiss icon seamless", "meteoswiss_icon_seamless"),
        ("meteoswiss icon ch1", "meteoswiss_icon_ch1"),
        ("meteoswiss icon ch2", "meteoswiss_icon_ch2"),
        ("ncep gfs seamless", "ncep_gfs_seamless"),
        ("ncep gfs global", "ncep_gfs_global"),
        ("ncep rrfs north america", "ncep_rrfs_north_america"),
        ("ncep rrfs conus", "ncep_rrfs_conus"),
        ("ncep nbm conus", "ncep_nbm_conus"),
        ("ncep aigfs 0p25", "ncep_aigfs025"),
        ("ncep hgefs 0p25 ensemble mean", "ncep_hgefs025_ensemble_mean"),
        ("gem seamless", "cmc_gem_seamless"),
        ("cmc gem global", "cmc_gem_gdps"),
        ("cmc gem regional", "cmc_gem_rdps"),
        ("cmc gem hrdps continental", "cmc_gem_hrdps"),
        ("cmc gem hrdps west", "cmc_gem_hrdps_west"),
        ("knmi seamless", "knmi_seamless")

    ]);

    models
        .get(model.to_lowercase().as_str())
        .unwrap_or(&"best_match")
        .to_string()
}

// Formats the URL for the API call 
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