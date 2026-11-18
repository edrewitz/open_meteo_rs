"""
This file is our Python interface with the client's Rust core.

(C) Eric J. Drewitz 2026
"""
from ._core import get_forecast as fetch_forecast


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
    
    df = fetch_forecast(latitude,
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