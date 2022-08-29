use std::env::current_dir;
use std::fs::File;
use std::path::{PathBuf};
use polars::prelude::*;

#[derive(Debug)]
pub struct NamedCoordinates {
    pub lat: f64,
    pub long: f64,
    pub name: String,
}

fn get_file_path() -> PathBuf {
    let relative_path = PathBuf::from("src/uk_airport_coords.csv");
    let mut absolute_path = current_dir().unwrap().clone();
    absolute_path = absolute_path.join(relative_path);
    absolute_path
}

fn get_df() -> DataFrame {
    let file_path = get_file_path();
    let file = File::open(file_path);
    CsvReader::new(file.unwrap())
        .infer_schema(Some(100))
        .has_header(true)
        .finish()
        .unwrap()
}

pub fn get_coordinates_from_csv() -> Vec<NamedCoordinates> {
    let df = get_df();
    let rows_len = df.shape().0;
    let coords_series = df
        .select_series(&["Longitude", "Latitude", "NAME"])
        .unwrap();

    let long_list = &coords_series[0].f64().unwrap();
    let lat_list = &coords_series[1].f64().unwrap();
    let name_list = &coords_series[2].utf8().unwrap();

    let mut named_points: Vec<NamedCoordinates> = vec![];

    for idx in 0..rows_len {
        named_points.push(
            NamedCoordinates {
                long: long_list.get(idx).unwrap(),
                lat: lat_list.get(idx).unwrap(),
                name: String::from(name_list.get(idx).unwrap()),
            },
        );
    }
    named_points
}
