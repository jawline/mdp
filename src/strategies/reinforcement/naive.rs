use matrix::{ Matrix, MapMatrix };
use process::{ Process, State, Agent };
use rand::{ thread_rng, Rng };

pub struct ReinforcementAgent {
	pub learned_reward: MapMatrix<f32>
}

impl<T: Matrix<f32>> Agent<T> for ReinforcementAgent {
	
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool {
		true
	}

	fn select_transfer(&self, from: usize, possible: &Vec<(usize, f32, f32)>) -> Option<usize> {

		if possible.len() == 0 {
			return None;
		}

		if possible.len() == 1 {
			return Some(possible[0].0);
		}

		let mut rng = thread_rng();
		let mut target = possible[rng.gen_range(0, possible.len())].0;

		let sum = possible
			.iter()
			.fold(0.0, |last, &(id, _, _)| 
				last + self.learned_reward.get(from, id).unwrap()
			);

		if sum != 0.0 {
			target = possible
				.iter()
				.max_by(|(id, _, _), (id2, _, _)| {
					self
						.learned_reward
						.get(from, *id)
						.unwrap()
						.partial_cmp(
							&self.learned_reward
								.get(from, *id2)
								.unwrap()
						).unwrap()
				})
				.unwrap().0;

			println!("Select {} -> {} from past", from, target);
		} else {
			println!("{} -> {} at random", from, target);
		}

		Some(target)
	}

	fn select_reward(&self, process: &Process<T>, from: usize, to: usize, success: bool) -> f32 {
		process.reward.get(from, to).unwrap_or_else(|| 0.0)
	}

	fn punish_action(&mut self, process: &Process<T>, state: &State, from: usize, to: usize, reward: f32, success: bool) {
		println!("Punished {} {} {}", from, to, reward);
		
		let old = self
			.learned_reward
			.get(from, to)
			.unwrap();

		let new = state.reward + reward;
		
		if old != 0.0 && old >= new {
			self
				.learned_reward
				.set(from, to, 0.0);
		} else {
			self.learned_reward.set(from, to, new);
		}
	}
}