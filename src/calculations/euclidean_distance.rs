#[derive(Debug, Copy, Clone)]
pub struct EuclideanPoint {
    pub x1: f64,
    pub x2: f64,
}

impl EuclideanPoint {
    pub fn distance(&self) -> f64 {
        self.x2 - self.x1
    }
}

pub unsafe fn calculate_eucleidian_distance(x: EuclideanPoint, y: EuclideanPoint) -> f64 {
    let x_distance: f64 = f64::powf(x.distance(), 2.0);
    let y_distance: f64 = f64::powf(y.distance(), 2.0);
    f64::powf(x_distance + y_distance, 0.5)
}
