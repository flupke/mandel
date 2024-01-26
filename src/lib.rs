use ndarray::Array1;

pub type Vector = Array1<f64>;

pub fn tuple_to_vector(tuple: (f32, f32)) -> Vector {
    Array1::from_vec(vec![tuple.0 as f64, tuple.1 as f64])
}

pub fn new_vector<T: Into<f64>>(x: T, y: T) -> Vector {
    Array1::from_vec(vec![x.into(), y.into()])
}
