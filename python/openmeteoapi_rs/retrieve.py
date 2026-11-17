"""
This file is our Python interface with the client's Rust core.

(C) Eric J. Drewitz 2026
"""
import pandas as pd

from ._core import get_forecast


def fetch_forecast(latitude,
                   longitude,
                   variables=["temperature_2m"],
                   model="ecmwf_ifs025",
                   days=7,
                   proxy=None,
                   to_pandas=True):
    
    """
    Retrieves our weather data from the Open-Meteo API
    """
    
    df = get_forecast(latitude,
                      longitude,
                      variables,
                      model,
                      days,
                      proxy)
    
    if to_pandas == True:
        df = df.to_pandas()
    else:
        pass
    
    return df