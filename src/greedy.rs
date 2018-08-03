use matrix::Matrix;
use process::{ Process, ProcessSteps };
use rand;

pub struct GreedyStep {}

impl<T: Matrix<f32>> ProcessSteps<T> for GreedyStep {
	
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool {
		match process.transition.get(from, to) {
			Some(probability) => probability != 0.0,
			None => false
		}
	}

	fn select_transfer(&self, possible: &Vec<(usize, f32, f32)>) -> Option<usize> {
		if possible.len() != 0 {
			match possible.iter().max_by(|(_, _, reward1), (_, _, reward2)| reward1.partial_cmp(reward2).unwrap()) {
				Some(&(node, _, _)) => Some(node),
				None => None
			}
		} else {
			None
		}
	}

	fn select_reward(&self, process: &Process<T>, from: usize, to: usize, success: bool) -> f32 {
		process.reward
			.get(from, to)
			.unwrap_or_else(|| 0.0)
	}
}