/*
A library that provides introspects global life expectancy data using Polars
*/

use polars::prelude::*;
pub const CSV_FILE: &str = "src/data/global-life-expt-2022.csv";

//read in a csv file
pub fn read_csv(path: &str) -> DataFrame {
    CsvReader::from_path(path).unwrap().finish().unwrap()
}

//print "n" rows of a dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

//do kmeans clustering of 2018, 2019, 2020 in the dataframe
//returns a dataframe with the cluster column added
//TBD for next time
//pub fn cluster(df: &DataFrame) -> DataFrame {
