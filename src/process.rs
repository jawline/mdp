use matrix::Matrix;

pub struct Process<T: Matrix<f32>> {
	pub states: usize,
	pub transition: T,
	pub reward: T,
	pub discount: f32
}

pub trait Agent<T: Matrix<f32>> {
	fn select_transfer(&self, from: usize, possible: &Vec<(usize, f32, f32)>) -> Option<usize>;
	fn try_transfer(&self, process: &Process<T>, from: usize, to: usize) -> bool;
	fn select_reward(&self, process: &Process<T>, from: usize, to: usize, success: bool) -> f32;
	fn punish_action(&mut self, process: &Process<T>, state: &State, from: usize, to: usize, reward: f32, success: bool);
}

pub struct State {
	pub node: usize,
	pub reward: f32
}

pub fn step_process<T: Matrix<f32>, R: Agent<T>>(state: &mut State, process: &Process<T>, step: &mut R) -> bool {

	let mut transition_list = Vec::new();

	for i in 0..process.states {
		let probability = process.transition.get(state.node, i).unwrap();

		if probability != 0.0 {
			transition_list.push((i, probability, process.reward.get(state.node, i).unwrap()));
		}
	}

	if cfg!(debug_assertions) {
	
		println!("Current State: {}", state.node);
		println!("Current Reward: {}", state.reward);
		println!("Possible Transitions: ");

		for (state, probability, reward) in &transition_list {
			println!("{}, {}, {}", state, probability, reward);
		}

	}

	loop {
		if let Some(node) = step.select_transfer(state.node, &transition_list) {
			let transferred = step.try_transfer(&process, state.node, node);
			let reward = step.select_reward(&process, state.node, node, transferred);
			step.punish_action(&process, &state, state.node, node, reward, transferred);
			state.reward += reward;
			
			if transferred {
				
				if cfg!(debug_assertions) {
					println!("Transferred to: {}", node);
				}

				state.node = node;
				return true;
			}

		} else {
			return false;
		}
	}
}