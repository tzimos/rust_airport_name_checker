use crate::calculations::euclidean_distance::{calculate_eucleidian_distance, EuclideanPoint};
use crate::coordinates::csv_coords::NamedCoordinates;
use crate::coords::Coordinates;

pub unsafe fn get_airport_name(user_coordinates: &Coordinates, csv_coordinates: &Vec<NamedCoordinates>) -> String {
    let mut dist = 1000.0;
    let mut airport_name = String::new();

    for csv_coord in csv_coordinates.iter() {
        let lat_point = EuclideanPoint { x1: user_coordinates.lat, x2: csv_coord.lat };
        let long_point = EuclideanPoint { x1: user_coordinates.long, x2: csv_coord.long };
        let new_dist = calculate_eucleidian_distance(lat_point, long_point);
        if new_dist < dist {
            dist = new_dist;
            airport_name = csv_coord.name.clone();
        }
    }
    airport_name
}