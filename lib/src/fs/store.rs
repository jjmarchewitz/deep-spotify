use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn write_df_to_csv(df: &mut DataFrame, csv_path: &PathBuf) -> PolarsResult<()> {
    let mut file = File::create(csv_path).expect("could not create CSV file");

    CsvWriter::new(&mut file)
        .has_header(true)
        .with_separator(b',')
        .finish(df)?;

    Ok(())
}