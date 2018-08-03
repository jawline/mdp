use matrix::Matrix;

pub struct Process<T: Matrix<f32>> {
	pub states: usize,
	pub transition: T,
	pub reward: T,
	pub discount: f32
}