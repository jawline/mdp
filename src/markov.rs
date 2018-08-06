use matrix::Matrix;
use process::{ Process, Agent, State };
use rand;

pub struct MarkovChainAgent {}

impl<T: Matrix<f32>> Agent<T> for MarkovChainAgent {
	
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool {
		match process.transition.get(from, to) {
			Some(probability) => probability != 0.0,
			None => false
		}
	}

	fn select_transfer(&self, from: usize, possible: &Vec<(usize, f32, f32)>) -> Option<usize> {
		if possible.len() != 0 {

			let mut random_choice = rand::random::<f32>();

			for &(node, probability, _) in possible {
				
				if random_choice <= probability {
					return Some(node);
				}

				random_choice -= probability;
			}

			None
		} else {
			None
		}
	}

	fn select_reward(&self, process: &Process<T>, from: usize, to: usize, success: bool) -> f32 {
		process.reward
			.get(from, to)
			.unwrap_or_else(|| 0.0)
	}

	fn punish_action(&mut self, process: &Process<T>, state: &State, from: usize, to: usize, reward: f32, success: bool) {}
}