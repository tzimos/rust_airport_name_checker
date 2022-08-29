extern crate core;

mod coordinates;

use crate::coordinates::{csv_coords::get_coordinates_from_csv, coords};

mod calculations;

use crate::calculations::utils::get_airport_name;
use crate::coordinates::csv_coords::NamedCoordinates;
use crate::coords::Coordinates;

fn main() {
    let user_coords: Coordinates = coords::get_coordinates_from_input();
    let coordinates: Vec<NamedCoordinates> = get_coordinates_from_csv();
    let name: String = unsafe { get_airport_name(&user_coords, &coordinates) };
    println!("Airport name is: {:#?}", name);
}