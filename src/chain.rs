use matrix::Matrix;
use process::{ Process, ProcessSteps };
use rand;

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
			let random_choice = rand::random::<f64>();

			for (node, probability) in possible {
				//TODO
			}

			None
		} else {
			None
		}
	}
}