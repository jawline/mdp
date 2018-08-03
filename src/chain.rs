use matrix::Matrix;
use process::{ Process, ProcessSteps };

pub type MarkovProcess<T: Matrix<f32>> = Process<T>;

pub struct MarkovStep {}

impl<T: Matrix<f32>> ProcessSteps<T> for MarkovStep {
	
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool {
		match process.transition.get(from, to) {
			Some(probability) => probability != 0.0,
			None => false
		}
	}

	fn select_transfer(&self, possible: &Vec<(usize, f32)>) -> Option<usize> {
		if possible.len() != 0 {
			let (idx, p) = possible[0];
			Some(idx)
		} else {
			None
		}
	}
}