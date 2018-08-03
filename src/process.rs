use matrix::Matrix;

pub struct Process<T: Matrix<f32>> {
	pub states: usize,
	pub transition: T,
	pub reward: T,
	pub discount: f32
}

pub trait ProcessSteps<T: Matrix<f32>> {
	fn try_transfer(process: &Process<T>, from: usize, to: usize) -> bool;
}

pub struct ProcessState {
	pub current_state: usize
}