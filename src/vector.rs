#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_squared(&self, other: &Vector) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        self.distance_squared(other).sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn length(&self) -> f64 {
        self.magnitude()
    }

    pub fn normalize(&self) -> Self {
        let mut length = self.magnitude();

        if length > 0f64 {
            length = 1_f64 / length;
        }

        Self::new(self.x * length, self.y * length)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

#[cfg(test)]
mod vector_test {
    use super::Vector;

    #[test]
    fn vector_magnitude_squared() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        assert_eq!(p0.magnitude_squared(), p0x * p0x + p0y * p0y);
    }

    #[test]
    fn vector_magnitude() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        assert_eq!(p0.magnitude(), (p0x * p0x + p0y * p0y).sqrt());
    }

    #[test]
    fn vector_scale() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);
        let scalar = 3f64;
        let p1 = p0.scale(scalar);

        assert_eq!(p1.x, p0x * scalar);
        assert_eq!(p1.y, p0y * scalar);

    }

    #[test]
    fn vector_length() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        assert_eq!(p0.length(), (p0x * p0x + p0y * p0y).sqrt());
    }

    #[test]
    fn vector_distance_squared() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        let p1x = 1.0f64;
        let p1y = 1.0f64;
        let p1 = Vector::new(p1x, p1y);

        assert_eq!(
            p0.distance_squared(&p1),
            (p0x - p1x).powi(2) + (p0y - p1y).powi(2)
        );
    }

    #[test]
    fn vector_distance() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        let p1x = 1.0f64;
        let p1y = 1.0f64;
        let p1 = Vector::new(p1x, p1y);

        assert_eq!(
            p0.distance_squared(&p1),
            ((p0x - p1x).powi(2) + (p0y - p1y).powi(2)).sqrt()
        );
    }

    #[test]
    fn vector_dot() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        let p1x = 2.0f64;
        let p1y = 2.0f64;
        let p1 = Vector::new(p1x, p1y);

        assert_eq!(p0.dot(&p1), p0x * p1x + p0y * p1y);
    }

    #[test]
    fn vector_normalize() {
        let p0x = 1.0f64;
        let p0y = 1.0f64;
        let p0 = Vector::new(p0x, p0y);

        let length = ((p0x * p0x) + (p0y * p0y)).sqrt();
        let normal_vector_x = p0x / length;
        let normal_vector_y = p0y / length;

        let normal_vector = p0.normalize();
        assert_eq!(normal_vector.x, normal_vector_x);
        assert_eq!(normal_vector.y, normal_vector_y);
    }
}
