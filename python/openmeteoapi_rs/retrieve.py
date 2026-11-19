"""
This file is our Python interface with the client's Rust core.

(C) Eric J. Drewitz 2026
"""
from ._core import get_hourly_forecast as _fetch_hourly_forecast


def get_hourly_forecast(
    latitude: float,
    longitude: float,
    variables: list[str] | None = ["temperature_2m"],
    model: str = "best match",
    days: int = 7,
    wind_speed_units: str = "miles per hour",
    temperature_units: str = "fahrenheit",
    precipitation_units: str = "inch",
    proxy: str | None = None,
    to_pandas: bool = True,
    ):
    
    """
    Retrieves our weather data from the Open-Meteo API
    
    **Arguments**
    
    *Required*
    latitude: float
        The latitude in decimal degrees of the point weather forecast.
    longitude: float
        The longitude in decimal degrees of the point weather forecast.
        
    *Optional*
    variables: list[str]
        A list of variables for the query.
        Default=["temperature_2m"]
        
        *Variables*
        
        "temperature_2m"
        "relative_humidity_2m"
        "dew_point_2m"
        "apparent_temperature"
        "precipitation_probability"
        "precipitation"
        "rain"
        "showers"
        "snowfall"
        "snow_depth"
        "weather_code"
        "surface_pressure"
        "pressure_msl"
        "cloud_cover"
        "cloud_cover_low"
        "cloud_cover_mid"
        "cloud_cover_high"
        "visibility"
        "evapotranspiration"
        "et0_fao_evapotranspiration"
        "vapour_pressure_deficit"
        "wind_speed_10m"
        "wind_speed_120m"
        "wind_speed_80m"
        "wind_speed_180m"
        "wind_direction_10m"
        "wind_direction_80m"
        "wind_direction_120m"
        "wind_direction_180m"
        "wind_gusts_10m"
        "temperature_80m"
        "temperature_120m"
        "temperature_180m"
        "soil_temperature_0cm"
        "soil_temperature_6cm"
        "soil_temperature_18cm"
        "soil_temperature_54cm"
        "soil_moisture_0_to_1cm"
        "soil_moisture_3_to_9cm"
        "soil_moisture_1_to_3cm"
        "soil_moisture_9_to_27cm"
        "soil_moisture_27_to_81cm"
        "uv_index"
        "uv_index_clear_sky"
        "is_day"
        "sunshine_duration"
        "wet_bulb_temperature_2m"
        "total_column_integrated_water_vapour"
        "cape"
        "lifted_index"
        "convective_inhibition"
        "freezing_level_height"
        "boundary_layer_height"
        "shortwave_radiation"
        "direct_radiation"
        "diffuse_radiation"
        "direct_normal_irradiance"
        "global_tilted_irradiance"
        "terrestrial_radiation"
        "shortwave_radiation_instant"
        "direct_radiation_instant"
        "diffuse_radiation_instant"
        "direct_normal_irradiance_instant"
        "global_tilted_irradiance_instant"
        "terrestrial_radiation_instant"
        "temperature_1000hPa"
        "temperature_975hPa"
        "temperature_950hPa"
        "temperature_925hPa"
        "temperature_900hPa"
        "temperature_850hPa"
        "temperature_800hPa"
        "temperature_700hPa"
        "temperature_600hPa"
        "temperature_400hPa"
        "temperature_300hPa"
        "temperature_500hPa"
        "temperature_250hPa"
        "temperature_200hPa"
        "temperature_150hPa"
        "temperature_100hPa"
        "temperature_70hPa"
        "temperature_50hPa"
        "temperature_30hPa"
        "dew_point_1000hPa"
        "dew_point_975hPa"
        "dew_point_950hPa"
        "dew_point_925hPa"
        "dew_point_900hPa"
        "dew_point_850hPa"
        "dew_point_800hPa"
        "dew_point_700hPa"
        "dew_point_600hPa"
        "dew_point_500hPa"
        "dew_point_400hPa"
        "dew_point_300hPa"
        "dew_point_250hPa"
        "dew_point_200hPa"
        "dew_point_150hPa"
        "dew_point_100hPa"
        "dew_point_70hPa"
        "dew_point_50hPa"
        "dew_point_30hPa"
        "relative_humidity_1000hPa"
        "relative_humidity_975hPa"
        "relative_humidity_950hPa"
        "relative_humidity_925hPa"
        "relative_humidity_900hPa"
        "relative_humidity_850hPa"
        "relative_humidity_800hPa"
        "relative_humidity_700hPa"
        "relative_humidity_600hPa"
        "relative_humidity_500hPa"
        "relative_humidity_400hPa"
        "relative_humidity_300hPa"
        "relative_humidity_250hPa"
        "relative_humidity_200hPa"
        "relative_humidity_150hPa"
        "relative_humidity_100hPa"
        "relative_humidity_70hPa"
        "relative_humidity_50hPa"
        "relative_humidity_30hPa"
        "cloud_cover_1000hPa"
        "cloud_cover_975hPa"
        "cloud_cover_950hPa"
        "cloud_cover_925hPa"
        "cloud_cover_900hPa"
        "cloud_cover_850hPa"
        "cloud_cover_800hPa"
        "cloud_cover_700hPa"
        "cloud_cover_600hPa"
        "cloud_cover_500hPa"
        "cloud_cover_400hPa"
        "cloud_cover_300hPa"
        "cloud_cover_250hPa"
        "cloud_cover_200hPa"
        "cloud_cover_150hPa"
        "cloud_cover_100hPa"
        "cloud_cover_70hPa"
        "cloud_cover_50hPa"
        "cloud_cover_30hPa"
        "wind_speed_1000hPa"
        "wind_speed_975hPa"
        "wind_speed_950hPa"
        "wind_speed_925hPa"
        "wind_speed_900hPa"
        "wind_speed_850hPa"
        "wind_speed_800hPa"
        "wind_speed_700hPa"
        "wind_speed_600hPa"
        "wind_speed_500hPa"
        "wind_speed_400hPa"
        "wind_speed_300hPa"
        "wind_speed_250hPa"
        "wind_speed_200hPa"
        "wind_speed_150hPa"
        "wind_speed_100hPa"
        "wind_speed_70hPa"
        "wind_speed_50hPa"
        "wind_speed_30hPa"
        "wind_direction_1000hPa"
        "wind_direction_975hPa"
        "wind_direction_950hPa"
        "wind_direction_925hPa"
        "wind_direction_900hPa"
        "wind_direction_850hPa"
        "wind_direction_800hPa"
        "wind_direction_700hPa"
        "wind_direction_600hPa"
        "wind_direction_500hPa"
        "wind_direction_400hPa"
        "wind_direction_300hPa"
        "wind_direction_250hPa"
        "wind_direction_200hPa"
        "wind_direction_150hPa"
        "wind_direction_100hPa"
        "wind_direction_70hPa"
        "wind_direction_50hPa"
        "wind_direction_30hPa"
        "geopotential_height_1000hPa"
        "geopotential_height_975hPa"
        "geopotential_height_950hPa"
        "geopotential_height_925hPa"
        "geopotential_height_900hPa"
        "geopotential_height_850hPa"
        "geopotential_height_800hPa"
        "geopotential_height_700hPa"
        "geopotential_height_600hPa"
        "geopotential_height_500hPa"
        "geopotential_height_400hPa"
        "geopotential_height_300hPa"
        "geopotential_height_250hPa"
        "geopotential_height_200hPa"
        "geopotential_height_150hPa"
        "geopotential_height_100hPa"
        "geopotential_height_70hPa"
        "geopotential_height_50hPa"
        "geopotential_height_30hPa"
        
    model: str
        Weather Forecast Model Selection.
        Default="best match"
        
        *Models*
        
        "best match"
        "ecmwf ifs hres 9km"
        "ecmwf ifs 0p25"
        "ecmwf aifs 0p25 single"
        "cma grapes global"
        "dwd icon seamless"
        "dwd icon global native"
        "dwd aicon global native"
        "dwd icon eu native"
        "dwd icon d2 native"
        "dwd icon d2 ruc native"
        "met norway seamless"
        "met norway nordic"
        "meteoswiss icon seamless"
        "meteoswiss icon ch1"
        "meteoswiss icon ch2"
        "ncep gfs seamless"
        "ncep gfs global"
        "ncep rrfs north america"
        "ncep rrfs conus"
        "ncep nbm conus"
        "ncep aigfs 0p25"
        "ncep hgefs 0p25 ensemble mean"
        "gem seamless"
        "cmc gem global"
        "cmc gem regional"
        "cmc gem hrdps continental"
        "cmc gem hrdps west"
        "knmi seamless"
        "knmi harmonie arome europe"
        "knmi harmonie arome netherlands"
        "geosphere seamless"
        "geosphere claef austria"
        "jma seamless"
        "jma msm"
        "jma gsm"
        "meteofrance seamless"
        "meteofrance arpege world"
        "meteofrance arpege europe"
        "meteofrance arome france"
        "meteofrance arome france hd"
        "dmi seamless"
        "dmi harmonie arome europe"
        "chmi aladin seamless"
        "chmi aladin central europe 2km"
        "chmi aladin cz 1km"
        "italia meteo arpae icon 2i"
        "uk met office seamless"
        "uk met office global 10km"
        "uk met office uk 2km"
        
    days: int
        The amount of days of the forecast.
        Default=7
        
    wind_speed_units: str
        The units for wind speed.
        Default="miles per hour"
        
        *Wind Speed Units*
        
        "kilometers per hour"
        "meters per second"
        "miles per hour"
        "knots"
        
    temperature_units: str
        The units for temperature.
        Default="fahrenheit"
        
        *Temperature Units*
        
        "fahrenheit"
        "celsius"
        
    precipitation_units: str
        The units for precipitation.
        Default="inch"
        
        *Precipitation Units*
        
        "inch"
        "millimeter"
        
    proxy: str or None
        The proxy server address and port for users working on a proxy server.
        Example: `proxy="https://proxy-server-address:proxy-server-port`
        Default=None
        
    to_pandas: bool
        to_pandas=True -> Pandas.DataFrame is returned
        to_pandas=False -> Polars.DataFrame is returned
        Default=True
        
    **Returns**
    
    A Pandas or Polars DataFrame of the requested hourly weather forecast data.   
    
    **Example**
        
    Retrieve the latest GFS 0.25x0.25 Degree Hourly Forecast
    Location: Islip Mac Arthur Airport (40.79°N/73.10°W)
    Variables: 
        1) 2-meter temperature
        2) 2-meter relative humidity
        3) precipitation
        4) 10-meter wind speed
    model: GFS 0.25x0.25 Degree
    days: 7-Day Forecast
    Wind Speed Units: Miles Per Hour
    Temperature Units: Fahrenheit
    Precipitation Units: Inches
    proxy: No Proxy
    
    ```python
    
    from openmeteoapi_rs import get_hourly_forecast
    
    latitude = 40.79
    longitude = 73.10
    variables = ["temperature_2m",
                 "relative_humidity_2m",
                 "precipitation",
                 "wind_speed_10m"]        
    model = "ncep gfs global"
    days = 7
    wind_speed_units = "miles per hour"
    temperature_units = "fahrenheit"
    precipitation_units = "inch"
    proxy = None
    to_pandas = True
    
    df = get_hourly_forecast(
    latitude,
    longitude,
    variables=variables,
    model=model,
    days=days,
    wind_speed_units=wind_speed_units,
    temperature_units=temperature_units,
    precipitation_units=precipitation_units,
    proxy=proxy,
    to_pandas=to_pandas,
    ):
    
    ```
     
    """
    
    df = _fetch_hourly_forecast(latitude,
                      longitude,
                      variables,
                      model,
                      days,
                      wind_speed_units,
                      temperature_units,
                      precipitation_units,
                      proxy)
    
    if to_pandas == True:
        df = df.to_pandas()
    else:
        pass
    
    return df