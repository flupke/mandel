use mandel::Vector;
use num::Complex;

pub struct View {
    screen_size: Vector,
    center: Vector,
    size: Vector,
}

impl View {
    pub fn new(screen_width: usize, screen_height: usize, center: Vector, size: Vector) -> Self {
        Self {
            screen_size: Vector::from_vec(vec![screen_width as f64, screen_height as f64]),
            center,
            size,
        }
    }

    pub fn map(&self, x: usize, y: usize) -> Complex<f64> {
        let vec = self.to_view_space(&Vector::from_vec(vec![x as f64, y as f64]));
        Complex::new(vec[0], vec[1])
    }

    pub fn pan(&mut self, screen_delta: &Vector) {
        let vec = screen_delta / &self.screen_size * &self.size;
        self.center -= &vec;
    }

    pub fn zoom(&mut self, center: &Vector, factor: f32) {
        let center = self.to_view_space(center);
        self.size *= factor as f64;
        self.center = &center + (&self.center - &center) * factor as f64;
    }

    pub fn to_view_space(&self, point: &Vector) -> Vector {
        point / &self.screen_size * &self.size - &self.size / 2.0 + &self.center
    }
}
