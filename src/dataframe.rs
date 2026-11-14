/// This file hosts the functions that clean up DataFrames
/// 
/// (C) Eric J. Drewitz 2026

use polars::prelude::*;

pub(crate) fn drop_empty_cols_and_rows(df: &DataFrame) -> PolarsResult<DataFrame> {
    let height = df.height();

    // 1. Keep columns that have at least one non-null value
    let non_empty_cols: Vec<&str> = df
        .columns()
        .iter()
        .filter(|col| col.null_count() < height)
        .map(|col| col.name().as_str())
        .collect();

    // If every column was empty, return an empty DataFrame
    if non_empty_cols.is_empty() {
        return Ok(DataFrame::empty());
    }

    // 2. Select valid columns, then keep rows where ANY column is not null
    df.select(non_empty_cols)?
        .lazy()
        .filter(any_horizontal([col("*").is_not_null()])?)
        .collect()
}