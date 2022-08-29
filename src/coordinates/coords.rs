use std::io::stdin;

#[derive(PartialEq)]
enum Coordinate {
    Latitude,
    Longitude,
}

#[derive(Debug)]
pub struct Coordinates {
    pub lat: f64,
    pub long: f64,
}

pub fn get_coordinates_from_input() -> Coordinates {
    let mut user_input = String::new();
    Coordinates {
        lat:  get_coordinate_from_input(Coordinate::Latitude, &mut user_input),
        long: get_coordinate_from_input(Coordinate::Longitude, &mut user_input)
    }
}

fn get_coordinate_from_input(coordinate: Coordinate, user_input: &mut String) -> f64 {
    let coord = if coordinate == Coordinate::Longitude { "longitude" } else { "latitude" };
    println!("Please enter a {:#?}: ", coord);
    stdin().read_line(user_input).unwrap();
    let coord_value: f64 = match user_input.trim().parse() {
        Ok(x) => {
            if coordinate == Coordinate::Latitude && ((-90.0 > x) || (x > 90.0)) {
                panic!("Latitude must be between -90 and 90")
            } else if coordinate == Coordinate::Longitude && ((-180.0 > x) || (x > 180.0)) {
                panic!("Longitude must be between -90 and 90")
            }
            x
        },
        Err(_err) => panic!("Panic: {:#?}", _err)
    };
    user_input.clear();
    coord_value
}
